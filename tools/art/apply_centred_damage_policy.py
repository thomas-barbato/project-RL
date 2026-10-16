"""Apply the accepted AOE-FALLOFF01 override after delegated spatial rank curves."""
BINDINGS = {
    'SG00': ('raw_physical_explosion_damage', 'radius_cells', 1, 'square'),
    'SM00': ('raw_fire_explosion_damage', 'radius_cells', 1, 'square'),
    'A02': ('raw_occult_damage_per_target', 'zone_width_cells', 2, 'square'),
    'FE04': ('raw_center_fire_damage', 'radius_cells', 1, 'square'),
    'FE05': ('raw_explosion_fire_damage', 'radius_cells', 1, 'disc'),
    'FR02': ('raw_cold_damage_per_target', 'radius_cells', 1, 'disc'),
    'FR05': ('raw_cold_damage_per_impact', 'radius_cells', 1, 'disc'),
    'FR06': ('raw_cold_damage_on_frozen_source', 'radius_cells', 1, 'diamond'),
    'HY07': ('raw_fire_explosion_damage', 'radius_cells', 1, 'diamond'),
    'TR64': ('raw_pulse_damage_per_target', 'pulse_radius_cells', 1, 'square'),
}
REPLACED = {'FE04':'maximum_edge_reduction_percent', 'FR06':'splash_percent_of_source_damage'}

def apply_policy(catalogue):
    for skill in catalogue['skills']:
        if skill['id'] not in BINDINGS:
            continue
        key, extent, divisor, shape = BINDINGS[skill['id']]
        assert key in skill['parameters'] and extent in skill['parameters']
        skill['centred_damage'] = {
            'review_id':'AOE-FALLOFF01', 'damage_key':key, 'extent_key':extent,
            'extent_divisor':divisor, 'shape':shape,
            'scope':'direct_damage_only;one_scale_before_defenses',
            'source_percent':100, 'edge_percent':50,
        }
        old_key = REPLACED.get(skill['id'])
        if old_key:
            skill['parameters'].pop(old_key, None)
    return catalogue
