//! Bounded first-layer template, not the topology of every future layer.
//! Reservations carry stable roles; opt-in local landscapes use those roles.
use super::{
    ContentId, RegionCoord, RegionVerticalLink, RegionalWorldDefinition, RegionalWorldError,
    coordinate_hash,
};

#[derive(Clone, PartialEq, Eq)]
pub struct FirstLayerPlanDefinition {
    pub city: ContentId,
    pub next_city: ContentId,
    pub landscapes: bool,
    pub encounters: bool,
}

impl std::fmt::Debug for FirstLayerPlanDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("FirstLayerPlanDefinition");
        d.field("city", &self.city)
            .field("next_city", &self.next_city);
        // Retain the exact v118 catalogue fingerprint when disabled.
        if self.landscapes {
            d.field("landscapes", &true);
        }
        if self.encounters {
            d.field("encounters", &true);
        }
        d.finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FirstLayerPlace {
    City,
    Workshops,
    Descent,
    Conduits,
    Pumps,
    Gallery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstLayerPlan {
    pub places: [(FirstLayerPlace, RegionCoord); 6],
    pub routes: [Vec<RegionCoord>; 2],
    pub cross_link: [RegionCoord; 2],
    pub descent: RegionVerticalLink,
    /// Landing, intermediate region, existing second-layer city.
    pub arrival_route: [RegionCoord; 3],
}

impl RegionalWorldDefinition {
    pub fn with_first_layer_plan(
        mut self,
        definition: Option<FirstLayerPlanDefinition>,
    ) -> Result<Self, RegionalWorldError> {
        if let Some(definition) = &definition {
            if definition.encounters && !definition.landscapes {
                return Err(RegionalWorldError::InvalidLayerPlan(
                    "local encounters require landscapes",
                ));
            }
            if definition.landscapes
                && (self.local_map_size.width() < 64 || self.local_map_size.height() < 48)
            {
                return Err(RegionalWorldError::InvalidLayerPlan(
                    "landscapes require at least 64 x 48 cells",
                ));
            }
            if definition.landscapes
                && self
                    .biomes
                    .iter()
                    .filter(|b| b.weight_at_depth(1) > 0)
                    .any(|b| {
                        !b.sites().is_empty()
                            || b.landmarks().cache_range().0 == 0
                            || b.loot().is_none_or(|loot| {
                                loot.minimum_draws() < b.landmarks().cache_range().1
                            })
                    })
            {
                return Err(RegionalWorldError::InvalidLayerPlan(
                    "landscapes require site-free layer-one biomes with budgeted cache rewards",
                ));
            }
            let city = self
                .cities
                .iter()
                .find(|c| c.id() == &definition.city)
                .ok_or(RegionalWorldError::InvalidLayerPlan(
                    "unknown starting city",
                ))?;
            let next = self
                .cities
                .iter()
                .find(|c| c.id() == &definition.next_city)
                .ok_or(RegionalWorldError::InvalidLayerPlan("unknown arrival city"))?;
            if city.coordinate().depth != 1 || next.coordinate().depth != 2 {
                return Err(RegionalWorldError::InvalidLayerPlan(
                    "template requires layers 1 and 2",
                ));
            }
            let original = RegionVerticalLink::new(city.coordinate(), next.coordinate())?;
            if !self.vertical_links.contains(&original) {
                return Err(RegionalWorldError::InvalidLayerPlan(
                    "missing original city link",
                ));
            }
            if !(0..8).any(|orientation| {
                self.first_layer_candidate(definition, orientation)
                    .is_some()
            }) {
                return Err(RegionalWorldError::InvalidLayerPlan(
                    "no unoccupied placement inside world bounds",
                ));
            }
        }
        self.first_layer_plan = definition;
        Ok(self)
    }

    pub fn first_layer_plan(&self, seed: u64) -> Option<FirstLayerPlan> {
        let definition = self.first_layer_plan.as_ref()?;
        let candidates = (0..8)
            .filter_map(|orientation| self.first_layer_candidate(definition, orientation))
            .collect::<Vec<_>>();
        let anchor = candidates.first()?.places[0].1;
        // Independent of biome, terrain, actor and loot draws. No mutable RNG.
        let roll = coordinate_hash(
            seed,
            anchor.x,
            anchor.y,
            anchor.depth,
            0x4c41_5945_525f_5031,
        );
        candidates
            .get((roll % candidates.len() as u64) as usize)
            .cloned()
    }

    /// Ephemeral topology for generation/navigation. Keep the authored catalogue
    /// (with its opt-in template) for fingerprints and replay compatibility.
    /// The returned definition has no unresolved template and is idempotent.
    pub fn resolved_for_seed(&self, seed: u64) -> Self {
        let mut world = self.clone();
        if let Some(plan) = self.first_layer_plan(seed) {
            world.planned_encounters = self.first_layer_plan.as_ref().is_some_and(|d| d.encounters);
            if self
                .first_layer_plan
                .as_ref()
                .is_some_and(|definition| definition.landscapes)
            {
                world.planned_landscapes = plan
                    .places
                    .into_iter()
                    .filter(|(role, _)| *role != FirstLayerPlace::City)
                    .map(|(role, coordinate)| (coordinate, role))
                    .collect();
            }
            let original = RegionVerticalLink::new(plan.places[0].1, plan.arrival_route[2])
                .expect("validated first-layer cities share their original shaft");
            let link = world
                .vertical_links
                .iter_mut()
                .find(|link| **link == original)
                .expect("validated first-layer original link exists");
            *link = plan.descent;
        }
        world.first_layer_plan = None;
        world
    }

    pub fn planned_landscape_at(&self, coordinate: RegionCoord) -> Option<FirstLayerPlace> {
        self.planned_landscapes.get(&coordinate).copied()
    }

    pub fn has_planned_encounters_at(&self, coordinate: RegionCoord) -> bool {
        self.planned_encounters && self.planned_landscapes.contains_key(&coordinate)
    }

    fn first_layer_candidate(
        &self,
        definition: &FirstLayerPlanDefinition,
        orientation: u8,
    ) -> Option<FirstLayerPlan> {
        let anchor = self
            .cities
            .iter()
            .find(|c| c.id() == &definition.city)?
            .coordinate();
        let next = self
            .cities
            .iter()
            .find(|c| c.id() == &definition.next_city)?
            .coordinate();
        let transform = |x: i32, y: i32| {
            let y = if orientation >= 4 { -y } else { y };
            match orientation % 4 {
                0 => (x, y),
                1 => (-y, x),
                2 => (-x, -y),
                _ => (y, -x),
            }
        };
        use FirstLayerPlace::*;
        let mut places = [(City, anchor); 6];
        for (index, (role, x, y)) in [
            (City, 0, 0),
            (Workshops, 1, 0),
            (Descent, 2, 0),
            (Conduits, 0, 1),
            (Pumps, 1, 1),
            (Gallery, 2, 1),
        ]
        .into_iter()
        .enumerate()
        {
            let (x, y) = transform(x, y);
            let coordinate = RegionCoord::new(
                anchor.x.checked_add(x)?,
                anchor.y.checked_add(y)?,
                anchor.depth,
            );
            if !self.bounds.contains(coordinate)
                || (role != City
                    && (self.city_at(coordinate).is_some()
                        || self
                            .vertical_links
                            .iter()
                            .any(|l| l.upper() == coordinate || l.lower() == coordinate)))
            {
                return None;
            }
            places[index] = (role, coordinate);
        }
        let landing = RegionCoord::new(places[2].1.x, places[2].1.y, next.depth);
        let intermediate = RegionCoord::new(places[1].1.x, places[1].1.y, next.depth);
        for coordinate in [landing, intermediate] {
            if !self.bounds.contains(coordinate)
                || self.city_at(coordinate).is_some()
                || self
                    .vertical_links
                    .iter()
                    .any(|l| l.upper() == coordinate || l.lower() == coordinate)
            {
                return None;
            }
        }
        Some(FirstLayerPlan {
            places,
            routes: [
                vec![places[0].1, places[1].1, places[2].1],
                vec![
                    places[0].1,
                    places[3].1,
                    places[4].1,
                    places[5].1,
                    places[2].1,
                ],
            ],
            cross_link: [places[1].1, places[4].1],
            descent: RegionVerticalLink::new(places[2].1, landing).ok()?,
            arrival_route: [landing, intermediate, next],
        })
    }
}
