def test_all_migrated_enums():
    """Test that all migrated enums can be imported and used."""
    from botbowl.rust_core.table import (
        ActionType,
        BBDieResult,
        CasualtyEffect,
        CasualtyType,
        OutcomeType,
        PassDistance,
        PhysicalState,
        PlayerActionType,
        RollType,
        Skill,
        SkillCategory,
        Tile,
        WeatherType,
    )

    # Test Tile enum
    assert Tile.HOME == 1
    assert Tile.HOME_TOUCHDOWN == 2
    assert Tile.MIDFIELD == 12

    # Test BBDieResult enum
    assert BBDieResult.ATTACKER_DOWN == 1
    assert BBDieResult.DEFENDER_DOWN == 5

    # Test RollType enum
    assert RollType.AGILITY_ROLL == 1
    assert RollType.LAND_ROLL == 34

    # Test OutcomeType enum
    assert OutcomeType.HEADS_WON == 1
    assert OutcomeType.ACTION_SELECT_DIE == 1000

    # Test PlayerActionType enum
    assert PlayerActionType.MOVE == 1
    assert PlayerActionType.THROW_BOMB == 7

    # Test PhysicalState enum
    assert PhysicalState.NONE == 1
    assert PhysicalState.EJECTED == 14

    # Test CasualtyEffect enum
    assert CasualtyEffect.NONE == 1
    assert CasualtyEffect.DEAD == 8

    # Test CasualtyType enum
    assert CasualtyType.BADLY_HURT == 38
    assert CasualtyType.DEAD == 61

    # Test ActionType enum
    assert ActionType.START_GAME == 1
    assert ActionType.UNDO == 66

    # Test WeatherType enum
    assert WeatherType.SWELTERING_HEAT == 1
    assert WeatherType.BLIZZARD == 5

    # Test SkillCategory enum
    assert SkillCategory.GENERAL == 0
    assert SkillCategory.EXTRAORDINARY == 5

    # Test Skill enum
    assert Skill.THICK_SKULL == 1
    assert Skill.TIMMMBER == 79

    # Test PassDistance enum
    assert PassDistance.QUICK_PASS == 1
    assert PassDistance.HAIL_MARY == 5


def test_rules_class():
    """Test that the Rules class works correctly."""
    from botbowl.rust_core.table import (
        CasualtyEffect,
        CasualtyType,
        PassDistance,
        PlayerActionType,
        Rules,
    )

    # Test pass matrix
    pass_matrix = Rules.pass_matrix
    assert len(pass_matrix) == 14
    assert len(pass_matrix[0]) == 14
    assert pass_matrix[0][0] == 0
    assert pass_matrix[13][13] == 5

    # Test agility table
    agility_table = Rules.agility_table
    assert len(agility_table) == 11
    assert agility_table[0] == 6
    assert agility_table[10] == 1

    # Test pass modifier
    assert Rules.get_pass_modifier(PassDistance.QUICK_PASS) == 1
    assert Rules.get_pass_modifier(PassDistance.SHORT_PASS) == 0
    assert Rules.get_pass_modifier(PassDistance.LONG_PASS) == -1
    assert Rules.get_pass_modifier(PassDistance.LONG_BOMB) == -2
    assert Rules.get_pass_modifier(PassDistance.HAIL_MARY) == 0

    # Test casualty effect
    assert Rules.get_casualty_effect(CasualtyType.BADLY_HURT) == CasualtyEffect.NONE
    assert Rules.get_casualty_effect(CasualtyType.BROKEN_RIBS) == CasualtyEffect.MNG
    assert Rules.get_casualty_effect(CasualtyType.DEAD) == CasualtyEffect.DEAD

    # Test miss next game
    miss_next_game = Rules.miss_next_game
    assert len(miss_next_game) == 6
    assert CasualtyEffect.MNG in miss_next_game

    # Test immovable action types
    immovable_actions = Rules.immovable_action_types
    assert len(immovable_actions) == 2
    assert PlayerActionType.BLOCK in immovable_actions
    assert PlayerActionType.THROW_BOMB in immovable_actions

    # Test pass player actions
    pass_actions = Rules.pass_player_actions
    assert len(pass_actions) == 2
    assert PlayerActionType.PASS in pass_actions
    assert PlayerActionType.THROW_BOMB in pass_actions
