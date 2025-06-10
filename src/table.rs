use pyo3::prelude::*;

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum Tile {
    #[pyo3(name = "HOME")]
    Home = 1,
    #[pyo3(name = "HOME_TOUCHDOWN")]
    HomeTouchdown = 2,
    #[pyo3(name = "HOME_WING_LEFT")]
    HomeWingLeft = 3,
    #[pyo3(name = "HOME_WING_RIGHT")]
    HomeWingRight = 4,
    #[pyo3(name = "HOME_SCRIMMAGE")]
    HomeScrimmage = 5,
    #[pyo3(name = "AWAY")]
    Away = 6,
    #[pyo3(name = "AWAY_TOUCHDOWN")]
    AwayTouchdown = 7,
    #[pyo3(name = "AWAY_WING_LEFT")]
    AwayWingLeft = 8,
    #[pyo3(name = "AWAY_WING_RIGHT")]
    AwayWingRight = 9,
    #[pyo3(name = "AWAY_SCRIMMAGE")]
    AwayScrimmage = 10,
    #[pyo3(name = "CROWD")]
    Crowd = 11,
    #[pyo3(name = "MIDFIELD")]
    Midfield = 12,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum BBDieResult {
    #[pyo3(name = "ATTACKER_DOWN")]
    AttackerDown = 1,
    #[pyo3(name = "BOTH_DOWN")]
    BothDown = 2,
    #[pyo3(name = "PUSH")]
    Push = 3,
    #[pyo3(name = "DEFENDER_STUMBLES")]
    DefenderStumbles = 4,
    #[pyo3(name = "DEFENDER_DOWN")]
    DefenderDown = 5,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum RollType {
    #[pyo3(name = "AGILITY_ROLL")]
    AgilityRoll = 1,
    #[pyo3(name = "STRENGTH_ROLL")]
    StrengthRoll = 2,
    #[pyo3(name = "ARMOR_ROLL")]
    ArmorRoll = 3,
    #[pyo3(name = "INJURY_ROLL")]
    InjuryRoll = 4,
    #[pyo3(name = "CASUALTY_ROLL")]
    CasualtyRoll = 5,
    #[pyo3(name = "SCATTER_ROLL")]
    ScatterRoll = 6,
    #[pyo3(name = "BOUNCE_ROLL")]
    BounceRoll = 7,
    #[pyo3(name = "THROWN_IN_ROLL")]
    ThrownInRoll = 8,
    #[pyo3(name = "BLOCK_ROLL")]
    BlockRoll = 9,
    #[pyo3(name = "DISTANCE_ROLL")]
    DistanceRoll = 10,
    #[pyo3(name = "WEATHER_ROLL")]
    WeatherRoll = 11,
    #[pyo3(name = "KICKOFF_ROLL")]
    KickoffRoll = 12,
    #[pyo3(name = "FANS_ROLL")]
    FansRoll = 13,
    #[pyo3(name = "RIOT_ROLL")]
    RiotRoll = 14,
    #[pyo3(name = "CHEERING_FANS_ROLL")]
    CheeringFansRoll = 15,
    #[pyo3(name = "BRILLIANT_COACHING_ROLL")]
    BrilliantCoachingRoll = 16,
    #[pyo3(name = "THROW_A_ROCK_ROLL")]
    ThrowARockRoll = 17,
    #[pyo3(name = "PITCH_INVASION_ROLL")]
    PitchInvasionRoll = 19,
    #[pyo3(name = "GFI_ROLL")]
    GfiRoll = 20,
    #[pyo3(name = "STAND_UP_ROLL")]
    StandUpRoll = 21,
    #[pyo3(name = "KO_READY_ROLL")]
    KoReadyRoll = 22,
    #[pyo3(name = "SWELTERING_HEAT_ROLL")]
    SwelteringHeatRoll = 23,
    #[pyo3(name = "BONE_HEAD_ROLL")]
    BoneHeadRoll = 24,
    #[pyo3(name = "REALLY_STUPID_ROLL")]
    ReallyStupidRoll = 25,
    #[pyo3(name = "WILD_ANIMAL_ROLL")]
    WildAnimalRoll = 26,
    #[pyo3(name = "LONER_ROLL")]
    LonerRoll = 27,
    #[pyo3(name = "REGENERATION_ROLL")]
    RegenerationRoll = 28,
    #[pyo3(name = "TAKE_ROOT_ROLL")]
    TakeRootRoll = 29,
    #[pyo3(name = "SHADOWING_ROLL")]
    ShadowingRoll = 30,
    #[pyo3(name = "BRIBE_ROLL")]
    BribeRoll = 31,
    #[pyo3(name = "BLOOD_LUST_ROLL")]
    BloodLustRoll = 32,
    #[pyo3(name = "BOMB_ROLL")]
    BombRoll = 33,
    #[pyo3(name = "LAND_ROLL")]
    LandRoll = 34,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum OutcomeType {
    #[pyo3(name = "HEADS_WON")]
    HeadsWon = 1,
    #[pyo3(name = "HEADS_LOSS")]
    HeadsLoss = 2,
    #[pyo3(name = "WEATHER_SWELTERING_HEAT")]
    WeatherSwelteringHeat = 3,
    #[pyo3(name = "WEATHER_VERY_SUNNY")]
    WeatherVerySunny = 4,
    #[pyo3(name = "WEATHER_NICE")]
    WeatherNice = 5,
    #[pyo3(name = "WEATHER_POURING_RAIN")]
    WeatherPouringRain = 6,
    #[pyo3(name = "WEATHER_BLIZZARD")]
    WeatherBlizzard = 7,
    #[pyo3(name = "PLAYER_PLACED")]
    PlayerPlaced = 9,
    #[pyo3(name = "ILLEGAL_SETUP_NUM")]
    IllegalSetupNum = 10,
    #[pyo3(name = "ILLEGAL_SETUP_SCRIMMAGE")]
    IllegalSetupScrimmage = 11,
    #[pyo3(name = "ILLEGAL_SETUP_WINGS")]
    IllegalSetupWings = 12,
    #[pyo3(name = "BALL_PLACED")]
    BallPlaced = 13,
    #[pyo3(name = "KICKOFF_GET_THE_REF")]
    KickoffGetTheRef = 14,
    #[pyo3(name = "KICKOFF_RIOT")]
    KickoffRiot = 15,
    #[pyo3(name = "KICKOFF_PERFECT_DEFENSE")]
    KickoffPerfectDefense = 16,
    #[pyo3(name = "KICKOFF_HIGH_KICK")]
    KickoffHighKick = 17,
    #[pyo3(name = "KICKOFF_CHEERING_FANS")]
    KickoffCheeringFans = 18,
    #[pyo3(name = "KICKOFF_CHANGING_WHEATHER")]
    KickoffChangingWeather = 19,
    #[pyo3(name = "KICKOFF_BRILLIANT_COACHING")]
    KickoffBrilliantCoaching = 20,
    #[pyo3(name = "KICKOFF_QUICK_SNAP")]
    KickoffQuickSnap = 21,
    #[pyo3(name = "KICKOFF_BLITZ")]
    KickoffBlitz = 22,
    #[pyo3(name = "KICKOFF_THROW_A_ROCK")]
    KickoffThrowARock = 23,
    #[pyo3(name = "KICKOFF_PITCH_INVASION")]
    KickoffPitchInvasion = 24,
    #[pyo3(name = "EXTRA_BRIBE")]
    ExtraBribe = 25,
    #[pyo3(name = "RIOT")]
    Riot = 26,
    #[pyo3(name = "HIGH_KICK")]
    HighKick = 27,
    #[pyo3(name = "EXTRA_REROLL")]
    ExtraReroll = 28,
    #[pyo3(name = "BRILLIANT_COACHING")]
    BrilliantCoaching = 29,
    #[pyo3(name = "PITCH_INVASION_ROLL")]
    PitchInvasionRoll = 32,
    #[pyo3(name = "KICK_IN_BOUNDS")]
    KickInBounds = 34,
    #[pyo3(name = "KICK_OUT_OF_BOUNDS")]
    KickOutOfBounds = 35,
    #[pyo3(name = "BALL_HIT_GROUND")]
    BallHitGround = 36,
    #[pyo3(name = "BALL_HIT_PLAYER")]
    BallHitPlayer = 37,
    #[pyo3(name = "SETUP_DONE")]
    SetupDone = 38,
    #[pyo3(name = "KNOCKED_DOWN")]
    KnockedDown = 39,
    #[pyo3(name = "ARMOR_BROKEN")]
    ArmorBroken = 40,
    #[pyo3(name = "ARMOR_NOT_BROKEN")]
    ArmorNotBroken = 41,
    #[pyo3(name = "STUNNED")]
    Stunned = 42,
    #[pyo3(name = "KNOCKED_OUT")]
    KnockedOut = 43,
    #[pyo3(name = "BADLY_HURT")]
    BadlyHurt = 44,
    #[pyo3(name = "INTERCEPTION")]
    Interception = 46,
    #[pyo3(name = "BALL_CAUGHT")]
    BallCaught = 47,
    #[pyo3(name = "FAILED_CATCH")]
    FailedCatch = 48,
    #[pyo3(name = "FAILED_DODGE")]
    FailedDodge = 49,
    #[pyo3(name = "SUCCESSFUL_DODGE")]
    SuccessfulDodge = 50,
    #[pyo3(name = "FAILED_GFI")]
    FailedGfi = 51,
    #[pyo3(name = "SUCCESSFUL_GFI")]
    SuccessfulGfi = 52,
    #[pyo3(name = "FAILED_PICKUP")]
    FailedPickup = 53,
    #[pyo3(name = "SUCCESSFUL_PICKUP")]
    SuccessfulPickup = 54,
    #[pyo3(name = "HANDOFF")]
    Handoff = 57,
    #[pyo3(name = "STUNNED_TURNED")]
    StunnedTurned = 59,
    #[pyo3(name = "END_PLAYER_TURN")]
    EndPlayerTurn = 60,
    #[pyo3(name = "MOVE_ACTION_STARTED")]
    MoveActionStarted = 61,
    #[pyo3(name = "BLOCK_ACTION_STARTED")]
    BlockActionStarted = 62,
    #[pyo3(name = "BLITZ_ACTION_STARTED")]
    BlitzActionStarted = 63,
    #[pyo3(name = "PASS_ACTION_STARTED")]
    PassActionStarted = 64,
    #[pyo3(name = "FOUL_ACTION_STARTED")]
    FoulActionStarted = 65,
    #[pyo3(name = "HANDOFF_ACTION_STARTED")]
    HandoffActionStarted = 66,
    #[pyo3(name = "END_OF_GAME")]
    EndOfGame = 67,
    #[pyo3(name = "END_OF_PREGAME")]
    EndOfPregame = 68,
    #[pyo3(name = "END_OF_TURN")]
    EndOfTurn = 69,
    #[pyo3(name = "END_OF_FIRST_HALF")]
    EndOfFirstHalf = 70,
    #[pyo3(name = "TOUCHDOWN")]
    Touchdown = 71,
    #[pyo3(name = "TURNOVER")]
    Turnover = 72,
    #[pyo3(name = "CASUALTY")]
    Casualty = 73,
    #[pyo3(name = "APOTHECARY_USED_KO")]
    ApothecaryUsedKo = 74,
    #[pyo3(name = "APOTHECARY_USED_CASUALTY")]
    ApothecaryUsedCasualty = 75,
    #[pyo3(name = "CASUALTY_APOTHECARY")]
    CasualtyApothecary = 76,
    #[pyo3(name = "DAUNTLESS_USED")]
    DauntlessUsed = 77,
    #[pyo3(name = "PUSHED_INTO_CROWD")]
    PushedIntoCrowd = 78,
    #[pyo3(name = "PUSHED")]
    Pushed = 79,
    #[pyo3(name = "ACCURATE_PASS")]
    AccuratePass = 80,
    #[pyo3(name = "INACCURATE_PASS")]
    InaccuratePass = 81,
    #[pyo3(name = "FUMBLE")]
    Fumble = 82,
    #[pyo3(name = "HOME_RECEIVE")]
    HomeReceive = 84,
    #[pyo3(name = "AWAY_RECEIVE")]
    AwayReceive = 85,
    #[pyo3(name = "TAILS_WON")]
    TailsWon = 86,
    #[pyo3(name = "TAILS_LOSS")]
    TailsLoss = 87,
    #[pyo3(name = "TOUCHBACK")]
    Touchback = 88,
    #[pyo3(name = "GAME_STARTED")]
    GameStarted = 90,
    #[pyo3(name = "BALL_SCATTER")]
    BallScatter = 91,
    #[pyo3(name = "SPECTATORS")]
    Spectators = 92,
    #[pyo3(name = "FAME")]
    Fame = 93,
    #[pyo3(name = "KICK_OPP_HALF")]
    KickOppHalf = 95,
    #[pyo3(name = "GENTLE_GUST_OUT_OF_BOUNDS")]
    GentleGustOutOfBounds = 96,
    #[pyo3(name = "GENTLE_GUST_IN_BOUNDS")]
    GentleGustInBounds = 97,
    #[pyo3(name = "GENTLE_GUST_OPP_HALF")]
    GentleGustOppHalf = 98,
    #[pyo3(name = "END_OF_BLITZ")]
    EndOfBlitz = 99,
    #[pyo3(name = "END_OF_QUICK_SNAP")]
    EndOfQuickSnap = 100,
    #[pyo3(name = "END_OF_SECOND_HALF")]
    EndOfSecondHalf = 101,
    #[pyo3(name = "PLAYER_PLACED_HIGH_KICK")]
    PlayerPlacedHighKick = 102,
    #[pyo3(name = "TOUCHBACK_BALL_PLACED")]
    TouchbackBallPlaced = 103,
    #[pyo3(name = "HIT_BY_ROCK")]
    HitByRock = 104,
    #[pyo3(name = "TURN_START")]
    TurnStart = 105,
    #[pyo3(name = "PLAYER_READY")]
    PlayerReady = 106,
    #[pyo3(name = "PLAYER_NOT_READY")]
    PlayerNotReady = 107,
    #[pyo3(name = "SUCCESSFUL_CATCH")]
    SuccessfulCatch = 108,
    #[pyo3(name = "SKILL_USED")]
    SkillUsed = 109,
    #[pyo3(name = "STAND_UP")]
    StandUp = 110,
    #[pyo3(name = "FAILED_STAND_UP")]
    FailedStandUp = 111,
    #[pyo3(name = "BALL_OUT_OF_BOUNDS")]
    BallOutOfBounds = 112,
    #[pyo3(name = "FOLLOW_UP")]
    FollowUp = 113,
    #[pyo3(name = "FOUL")]
    Foul = 114,
    #[pyo3(name = "PLAYER_EJECTED")]
    PlayerEjected = 115,
    #[pyo3(name = "MISS_NEXT_GAME")]
    MissNextGame = 116,
    #[pyo3(name = "DEAD")]
    Dead = 117,
    #[pyo3(name = "REROLL_USED")]
    RerollUsed = 118,
    #[pyo3(name = "BLOCK_ROLL")]
    BlockRoll = 119,
    #[pyo3(name = "FAILED_INTERCEPTION")]
    FailedInterception = 120,
    #[pyo3(name = "TURN_ADDED")]
    TurnAdded = 121,
    #[pyo3(name = "TURN_SKIPPED")]
    TurnSkipped = 122,
    #[pyo3(name = "BALL_BOUNCED")]
    BallBounced = 123,
    #[pyo3(name = "THROW_IN")]
    ThrowIn = 124,
    #[pyo3(name = "THROW_IN_OUT_OF_BOUNDS")]
    ThrowInOutOfBounds = 125,
    #[pyo3(name = "TEAM_SPECTATORS")]
    TeamSpectators = 126,
    #[pyo3(name = "CHEERING_FANS_ROLL")]
    CheeringFansRoll = 127,
    #[pyo3(name = "BRILLIANT_COACHING_ROLL")]
    BrilliantCoachingRoll = 128,
    #[pyo3(name = "THROW_A_ROCK_ROLL")]
    ThrowARockRoll = 129,
    #[pyo3(name = "QUICK_SNAP_START")]
    QuickSnapStart = 130,
    #[pyo3(name = "BLITZ_START")]
    BlitzStart = 131,
    #[pyo3(name = "END_OF_GAME_WINNER")]
    EndOfGameWinner = 132,
    #[pyo3(name = "END_OF_GAME_DRAW")]
    EndOfGameDraw = 133,
    #[pyo3(name = "PLAYER_HEATED")]
    PlayerHeated = 134,
    #[pyo3(name = "PLAYER_NOT_HEATED")]
    PlayerNotHeated = 135,
    #[pyo3(name = "END_OF_GAME_DISQUALIFICATION")]
    EndOfGameDisqualification = 136,
    #[pyo3(name = "FAILED_BONE_HEAD")]
    FailedBoneHead = 137,
    #[pyo3(name = "SUCCESSFUL_BONE_HEAD")]
    SuccessfulBoneHead = 138,
    #[pyo3(name = "FAILED_REALLY_STUPID")]
    FailedReallyStupid = 139,
    #[pyo3(name = "SUCCESSFUL_REALLY_STUPID")]
    SuccessfulReallyStupid = 140,
    #[pyo3(name = "FAILED_WILD_ANIMAL")]
    FailedWildAnimal = 141,
    #[pyo3(name = "SUCCESSFUL_WILD_ANIMAL")]
    SuccessfulWildAnimal = 142,
    #[pyo3(name = "SUCCESSFUL_LONER")]
    SuccessfulLoner = 143,
    #[pyo3(name = "FAILED_LONER")]
    FailedLoner = 144,
    #[pyo3(name = "SUCCESSFUL_PRO")]
    SuccessfulPro = 145,
    #[pyo3(name = "FAILED_PRO")]
    FailedPro = 146,
    #[pyo3(name = "FAILED_REGENERATION")]
    FailedRegeneration = 147,
    #[pyo3(name = "SUCCESSFUL_REGENERATION")]
    SuccessfulRegeneration = 148,
    #[pyo3(name = "SUCCESSFUL_LEAP")]
    SuccessfulLeap = 149,
    #[pyo3(name = "FAILED_LEAP")]
    FailedLeap = 150,
    #[pyo3(name = "SUCCESSFUL_TAKE_ROOT")]
    SuccessfulTakeRoot = 151,
    #[pyo3(name = "FAILED_TAKE_ROOT")]
    FailedTakeRoot = 152,
    #[pyo3(name = "JUMP_UP")]
    JumpUp = 153,
    #[pyo3(name = "FAILED_JUMP_UP")]
    FailedJumpUp = 154,
    #[pyo3(name = "DECAYING")]
    Decaying = 155,
    #[pyo3(name = "BRIBE_USED")]
    BribeUsed = 156,
    #[pyo3(name = "SUCCESSFUL_BRIBE")]
    SuccessfulBribe = 157,
    #[pyo3(name = "FAILED_BRIBE")]
    FailedBribe = 158,
    #[pyo3(name = "SUCCESSFUL_HYPNOTIC_GAZE")]
    SuccessfulHypnoticGaze = 159,
    #[pyo3(name = "FAILED_HYPNOTIC_GAZE")]
    FailedHypnoticGaze = 160,
    #[pyo3(name = "SUCCESSFUL_BLOOD_LUST")]
    SuccessfulBloodLust = 161,
    #[pyo3(name = "FAILED_BLOOD_LUST")]
    FailedBloodLust = 162,
    #[pyo3(name = "EJECTED_BY_BLOOD_LUST")]
    EjectedByBloodLust = 163,
    #[pyo3(name = "EATEN_DURING_BLOOD_LUST")]
    EatenDuringBloodLust = 164,
    #[pyo3(name = "BOMB_HIT")]
    BombHit = 165,
    #[pyo3(name = "BOMB_EXPLODED")]
    BombExploded = 166,
    #[pyo3(name = "SUCCESSFUL_LAND")]
    SuccessfulLand = 167,
    #[pyo3(name = "FAILED_LAND")]
    FailedLand = 168,
    #[pyo3(name = "PLAYER_BOUNCED")]
    PlayerBounced = 171,
    #[pyo3(name = "BOMB_OUT_OF_BOUNDS")]
    BombOutOfBounds = 169,
    #[pyo3(name = "PLAYER_OUT_OF_BOUNDS")]
    PlayerOutOfBounds = 170,
    #[pyo3(name = "BALL_BOUNCE_PLAYER")]
    BallBouncePlayer = 172,
    #[pyo3(name = "PLAYER_BOUNCE_PLAYER")]
    PlayerBouncePlayer = 173,
    #[pyo3(name = "BOMB_ON_GROUND")]
    BombOnGround = 175,
    #[pyo3(name = "BALL_BOUNCE_GROUND")]
    BallBounceGround = 174,
    #[pyo3(name = "PLAYER_BOUNCE_GROUND")]
    PlayerBounceGround = 176,
    #[pyo3(name = "FAILED_CATCH_BOMB")]
    FailedCatchBomb = 177,
    #[pyo3(name = "SUCCESSFUL_CATCH_BOMB")]
    SuccessfulCatchBomb = 178,
    #[pyo3(name = "WILL_CATCH_BOMB")]
    WillCatchBomb = 179,
    #[pyo3(name = "WONT_CATCH_BOMB")]
    WontCatchBomb = 180,
    #[pyo3(name = "SUCCESSFUL_ESCAPE_BEING_EATEN")]
    SuccessfulEscapeBeingEaten = 181,
    #[pyo3(name = "FAILED_ESCAPE_BEING_EATEN")]
    FailedEscapeBeingEaten = 182,
    #[pyo3(name = "SUCCESSFUL_ALWAYS_HUNGRY")]
    SuccessfulAlwaysHungry = 183,
    #[pyo3(name = "FAILED_ALWAYS_HUNGRY")]
    FailedAlwaysHungry = 184,
    #[pyo3(name = "PLAYER_SCATTER")]
    PlayerScatter = 185,
    #[pyo3(name = "BOMB_SCATTER")]
    BombScatter = 186,
    #[pyo3(name = "THROW_BOMB_ACTION_STARTED")]
    ThrowBombActionStarted = 187,
    #[pyo3(name = "PLAYER_HIT_PLAYER")]
    PlayerHitPlayer = 188,
    #[pyo3(name = "EATEN_DURING_ALWAYS_HUNGRY")]
    EatenDuringAlwaysHungry = 189,
    #[pyo3(name = "ACTION_SELECT_DIE")]
    ActionSelectDie = 1000,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum PlayerActionType {
    #[pyo3(name = "MOVE")]
    Move = 1,
    #[pyo3(name = "BLOCK")]
    Block = 2,
    #[pyo3(name = "BLITZ")]
    Blitz = 3,
    #[pyo3(name = "PASS")]
    Pass = 4,
    #[pyo3(name = "HANDOFF")]
    Handoff = 5,
    #[pyo3(name = "FOUL")]
    Foul = 6,
    #[pyo3(name = "THROW_BOMB")]
    ThrowBomb = 7,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum PhysicalState {
    #[pyo3(name = "NONE")]
    None = 1,
    #[pyo3(name = "STUNNED")]
    Stunned = 5,
    #[pyo3(name = "KOD")]
    Kod = 6,
    #[pyo3(name = "BH")]
    Bh = 7,
    #[pyo3(name = "MNG")]
    Mng = 8,
    #[pyo3(name = "DEAD")]
    Dead = 9,
    #[pyo3(name = "BONE_HEADED")]
    BoneHeaded = 10,
    #[pyo3(name = "HYPNOTIZED")]
    Hypnotized = 11,
    #[pyo3(name = "REALLY_STUPID")]
    ReallyStupid = 12,
    #[pyo3(name = "HEATED")]
    Heated = 13,
    #[pyo3(name = "EJECTED")]
    Ejected = 14,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum CasualtyEffect {
    #[pyo3(name = "NONE")]
    None = 1,
    #[pyo3(name = "MNG")]
    Mng = 2,
    #[pyo3(name = "NIGGLING")]
    Niggling = 3,
    #[pyo3(name = "MA")]
    Ma = 4,
    #[pyo3(name = "AV")]
    Av = 5,
    #[pyo3(name = "AG")]
    Ag = 6,
    #[pyo3(name = "ST")]
    St = 7,
    #[pyo3(name = "DEAD")]
    Dead = 8,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum CasualtyType {
    #[pyo3(name = "BADLY_HURT")]
    BadlyHurt = 38,
    #[pyo3(name = "BROKEN_RIBS")]
    BrokenRibs = 41,
    #[pyo3(name = "GROIN_STRAIN")]
    GroinStrain = 42,
    #[pyo3(name = "GOUGED_EYE")]
    GougedEye = 43,
    #[pyo3(name = "BROKEN_JAW")]
    BrokenJaw = 44,
    #[pyo3(name = "FRACTURED_ARM")]
    FracturedArm = 45,
    #[pyo3(name = "FRACTURED_LEG")]
    FracturedLeg = 46,
    #[pyo3(name = "SMASHED_HAND")]
    SmashedHand = 47,
    #[pyo3(name = "PINCHED_NERVE")]
    PinchedNerve = 48,
    #[pyo3(name = "DAMAGED_BACK")]
    DamagedBack = 51,
    #[pyo3(name = "SMASHED_KNEE")]
    SmashedKnee = 52,
    #[pyo3(name = "SMASHED_HIP")]
    SmashedHip = 53,
    #[pyo3(name = "SMASHED_ANKLE")]
    SmashedAnkle = 54,
    #[pyo3(name = "SERIOUS_CONCUSSION")]
    SeriousConcussion = 55,
    #[pyo3(name = "FRACTURED_SKULL")]
    FracturedSkull = 56,
    #[pyo3(name = "BROKEN_NECK")]
    BrokenNeck = 57,
    #[pyo3(name = "SMASHED_COLLAR_BONE")]
    SmashedCollarBone = 58,
    #[pyo3(name = "DEAD")]
    Dead = 61,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum ActionType {
    #[pyo3(name = "START_GAME")]
    StartGame = 1,
    #[pyo3(name = "HEADS")]
    Heads = 3,
    #[pyo3(name = "TAILS")]
    Tails = 4,
    #[pyo3(name = "KICK")]
    Kick = 5,
    #[pyo3(name = "RECEIVE")]
    Receive = 6,
    #[pyo3(name = "PLACE_PLAYER")]
    PlacePlayer = 7,
    #[pyo3(name = "END_SETUP")]
    EndSetup = 8,
    #[pyo3(name = "PLACE_BALL")]
    PlaceBall = 9,
    #[pyo3(name = "START_MOVE")]
    StartMove = 10,
    #[pyo3(name = "START_BLOCK")]
    StartBlock = 11,
    #[pyo3(name = "START_BLITZ")]
    StartBlitz = 12,
    #[pyo3(name = "START_PASS")]
    StartPass = 13,
    #[pyo3(name = "START_FOUL")]
    StartFoul = 14,
    #[pyo3(name = "START_HANDOFF")]
    StartHandoff = 15,
    #[pyo3(name = "END_PLAYER_TURN")]
    EndPlayerTurn = 16,
    #[pyo3(name = "MOVE")]
    Move = 17,
    #[pyo3(name = "BLOCK")]
    Block = 18,
    #[pyo3(name = "PASS")]
    Pass = 20,
    #[pyo3(name = "FOUL")]
    Foul = 21,
    #[pyo3(name = "HANDOFF")]
    Handoff = 22,
    #[pyo3(name = "USE_REROLL")]
    UseReroll = 24,
    #[pyo3(name = "END_TURN")]
    EndTurn = 25,
    #[pyo3(name = "USE_APOTHECARY")]
    UseApothecary = 27,
    #[pyo3(name = "USE_SKILL")]
    UseSkill = 28,
    #[pyo3(name = "DONT_USE_SKILL")]
    DontUseSkill = 29,
    #[pyo3(name = "CONTINUE")]
    Continue = 35,
    #[pyo3(name = "SELECT_PLAYER")]
    SelectPlayer = 37,
    #[pyo3(name = "SELECT_NONE")]
    SelectNone = 38,
    #[pyo3(name = "DONT_USE_APOTHECARY")]
    DontUseApothecary = 40,
    #[pyo3(name = "DONT_USE_REROLL")]
    DontUseReroll = 42,
    #[pyo3(name = "SELECT_FIRST_ROLL")]
    SelectFirstRoll = 41,
    #[pyo3(name = "SELECT_SECOND_ROLL")]
    SelectSecondRoll = 45,
    #[pyo3(name = "STAND_UP")]
    StandUp = 43,
    #[pyo3(name = "PUSH")]
    Push = 44,
    #[pyo3(name = "SELECT_ATTACKER_DOWN")]
    SelectAttackerDown = 46,
    #[pyo3(name = "SELECT_BOTH_DOWN")]
    SelectBothDown = 47,
    #[pyo3(name = "SELECT_PUSH")]
    SelectPush = 48,
    #[pyo3(name = "SELECT_DEFENDER_STUMBLES")]
    SelectDefenderStumbles = 49,
    #[pyo3(name = "SELECT_DEFENDER_DOWN")]
    SelectDefenderDown = 50,
    #[pyo3(name = "SETUP_FORMATION_WEDGE")]
    SetupFormationWedge = 51,
    #[pyo3(name = "SETUP_FORMATION_LINE")]
    SetupFormationLine = 52,
    #[pyo3(name = "SETUP_FORMATION_ZONE")]
    SetupFormationZone = 53,
    #[pyo3(name = "SETUP_FORMATION_SPREAD")]
    SetupFormationSpread = 54,
    #[pyo3(name = "FOLLOW_UP")]
    FollowUp = 55,
    #[pyo3(name = "LEAP")]
    Leap = 56,
    #[pyo3(name = "STAB")]
    Stab = 57,
    #[pyo3(name = "USE_BRIBE")]
    UseBribe = 58,
    #[pyo3(name = "DONT_USE_BRIBE")]
    DontUseBribe = 59,
    #[pyo3(name = "HYPNOTIC_GAZE")]
    HypnoticGaze = 60,
    #[pyo3(name = "START_THROW_BOMB")]
    StartThrowBomb = 61,
    #[pyo3(name = "THROW_BOMB")]
    ThrowBomb = 62,
    #[pyo3(name = "CATCH_BOMB")]
    CatchBomb = 63,
    #[pyo3(name = "DONT_CATCH_BOMB")]
    DontCatchBomb = 67,
    #[pyo3(name = "PICKUP_TEAM_MATE")]
    PickupTeamMate = 64,
    #[pyo3(name = "THROW_TEAM_MATE")]
    ThrowTeamMate = 65,
    #[pyo3(name = "UNDO")]
    Undo = 66,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum WeatherType {
    #[pyo3(name = "SWELTERING_HEAT")]
    SwelteringHeat = 1,
    #[pyo3(name = "VERY_SUNNY")]
    VerySunny = 2,
    #[pyo3(name = "NICE")]
    Nice = 3,
    #[pyo3(name = "POURING_RAIN")]
    PouringRain = 4,
    #[pyo3(name = "BLIZZARD")]
    Blizzard = 5,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum SkillCategory {
    #[pyo3(name = "GENERAL")]
    General = 0,
    #[pyo3(name = "AGILITY")]
    Agility = 1,
    #[pyo3(name = "STRENGTH")]
    Strength = 2,
    #[pyo3(name = "PASSING")]
    Passing = 3,
    #[pyo3(name = "MUTATION")]
    Mutation = 4,
    #[pyo3(name = "EXTRAORDINARY")]
    Extraordinary = 5,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum Skill {
    #[pyo3(name = "THICK_SKULL")]
    ThickSkull = 1,
    #[pyo3(name = "STUNTY")]
    Stunty = 2,
    #[pyo3(name = "MIGHTY_BLOW")]
    MightyBlow = 3,
    #[pyo3(name = "CLAWS")]
    Claws = 4,
    #[pyo3(name = "SPRINT")]
    Sprint = 5,
    #[pyo3(name = "SURE_FEET")]
    SureFeet = 6,
    #[pyo3(name = "NO_HANDS")]
    NoHands = 7,
    #[pyo3(name = "BALL_AND_CHAIN")]
    BallAndChain = 8,
    #[pyo3(name = "DODGE")]
    Dodge = 9,
    #[pyo3(name = "PREHENSILE_TAIL")]
    PrehensileeTail = 10,
    #[pyo3(name = "TACKLE")]
    Tackle = 11,
    #[pyo3(name = "BREAK_TACKLE")]
    BreakTackle = 12,
    #[pyo3(name = "TITCHY")]
    Titchy = 13,
    #[pyo3(name = "DIVING_TACKLE")]
    DivingTackle = 14,
    #[pyo3(name = "SHADOWING")]
    Shadowing = 15,
    #[pyo3(name = "TENTACLES")]
    Tentacles = 16,
    #[pyo3(name = "TWO_HEADS")]
    TwoHeads = 17,
    #[pyo3(name = "BLOCK")]
    Block = 18,
    #[pyo3(name = "WRESTLE")]
    Wrestle = 19,
    #[pyo3(name = "STAND_FIRM")]
    StandFirm = 20,
    #[pyo3(name = "GUARD")]
    Guard = 21,
    #[pyo3(name = "HORNS")]
    Horns = 22,
    #[pyo3(name = "SIDE_STEP")]
    SideStep = 23,
    #[pyo3(name = "FRENZY")]
    Frenzy = 24,
    #[pyo3(name = "CATCH")]
    Catch = 25,
    #[pyo3(name = "SURE_HANDS")]
    SureHands = 26,
    #[pyo3(name = "BIG_HAND")]
    BigHand = 27,
    #[pyo3(name = "EXTRA_ARMS")]
    ExtraArms = 28,
    #[pyo3(name = "DIRTY_PLAYER")]
    DirtyPlayer = 29,
    #[pyo3(name = "SNEAKY_GIT")]
    SneakyGit = 30,
    #[pyo3(name = "STRONG_ARM")]
    StrongArm = 31,
    #[pyo3(name = "LONG_LEGS")]
    LongLegs = 32,
    #[pyo3(name = "PASS")]
    Pass = 33,
    #[pyo3(name = "LONER")]
    Loner = 34,
    #[pyo3(name = "WILD_ANIMAL")]
    WildAnimal = 35,
    #[pyo3(name = "RIGHT_STUFF")]
    RightStuff = 36,
    #[pyo3(name = "ALWAYS_HUNGRY")]
    AlwaysHungry = 37,
    #[pyo3(name = "THROW_TEAM_MATE")]
    ThrowTeamMate = 38,
    #[pyo3(name = "BONE_HEAD")]
    BoneHead = 39,
    #[pyo3(name = "DUMP_OFF")]
    DumpOff = 40,
    #[pyo3(name = "STAB")]
    Stab = 41,
    #[pyo3(name = "JUMP_UP")]
    JumpUp = 42,
    #[pyo3(name = "DAUNTLESS")]
    Dauntless = 43,
    #[pyo3(name = "JUGGERNAUT")]
    Juggernaut = 44,
    #[pyo3(name = "SECRET_WEAPON")]
    SecretWeapon = 45,
    #[pyo3(name = "NERVES_OF_STEEL")]
    NervesOfSteel = 46,
    #[pyo3(name = "BOMBARDIER")]
    Bombardier = 47,
    #[pyo3(name = "LEAP")]
    Leap = 48,
    #[pyo3(name = "VERY_LONG_LEGS")]
    VeryLongLegs = 49,
    #[pyo3(name = "CHAINSAW")]
    Chainsaw = 50,
    #[pyo3(name = "TAKE_ROOT")]
    TakeRoot = 51,
    #[pyo3(name = "SAFE_THROW")]
    SafeThrow = 52,
    #[pyo3(name = "DECAY")]
    Decay = 53,
    #[pyo3(name = "DISTURBING_PRESENCE")]
    DisturbingPresence = 54,
    #[pyo3(name = "NURGLES_ROT")]
    NurglesRot = 55,
    #[pyo3(name = "FOUL_APPEARANCE")]
    FoulAppearance = 56,
    #[pyo3(name = "DIVING_CATCH")]
    DivingCatch = 57,
    #[pyo3(name = "BLOOD_LUST")]
    BloodLust = 58,
    #[pyo3(name = "HYPNOTIC_GAZE")]
    HypnoticGaze = 59,
    #[pyo3(name = "HAIL_MARY_PASS")]
    HailMaryPass = 60,
    #[pyo3(name = "ACCURATE")]
    Accurate = 61,
    #[pyo3(name = "KICK")]
    Kick = 62,
    #[pyo3(name = "KICK_OFF_RETURN")]
    KickOffReturn = 63,
    #[pyo3(name = "PASS_BLOCK")]
    PassBlock = 64,
    #[pyo3(name = "FEND")]
    Fend = 65,
    #[pyo3(name = "MULTIPLE_BLOCK")]
    MultipleBlock = 66,
    #[pyo3(name = "STRIP_BALL")]
    StripBall = 67,
    #[pyo3(name = "GRAB")]
    Grab = 68,
    #[pyo3(name = "STAKES")]
    Stakes = 69,
    #[pyo3(name = "ANIMOSITY")]
    Animosity = 70,
    #[pyo3(name = "PILING_ON")]
    PilingOn = 71,
    #[pyo3(name = "REALLY_STUPID")]
    ReallyStupid = 72,
    #[pyo3(name = "REGENERATION")]
    Regeneration = 73,
    #[pyo3(name = "MONSTROUS_MOUTH")]
    MonstrousMouth = 74,
    #[pyo3(name = "SWOOP")]
    Swoop = 75,
    #[pyo3(name = "FAN_FAVOURITE")]
    FanFavourite = 76,
    #[pyo3(name = "SWIFT_REACTION")]
    SwiftReaction = 77,
    #[pyo3(name = "PRO")]
    Pro = 78,
    #[pyo3(name = "TIMMMBER")]
    Timmmber = 79,
}

#[pyclass(eq, eq_int)]
#[derive(PartialEq, Clone)]
pub enum PassDistance {
    #[pyo3(name = "QUICK_PASS")]
    QuickPass = 1,
    #[pyo3(name = "SHORT_PASS")]
    ShortPass = 2,
    #[pyo3(name = "LONG_PASS")]
    LongPass = 3,
    #[pyo3(name = "LONG_BOMB")]
    LongBomb = 4,
    #[pyo3(name = "HAIL_MARY")]
    HailMary = 5,
}

#[pyclass]
pub struct Rules;

#[pymethods]
impl Rules {
    #[classattr]
    fn pass_matrix() -> Vec<Vec<i32>> {
        vec![
            vec![0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4],
            vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4],
            vec![1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 5],
            vec![1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5],
            vec![2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5],
            vec![2, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 5, 5],
            vec![2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 5, 5],
            vec![3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 5, 5, 5],
            vec![3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5],
            vec![3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5],
            vec![3, 3, 3, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5],
            vec![4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5],
            vec![4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5],
            vec![4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        ]
    }

    #[classattr]
    fn agility_table() -> Vec<i32> {
        vec![6, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1]
    }

    #[staticmethod]
    fn get_pass_modifier(distance: PassDistance) -> i32 {
        match distance {
            PassDistance::QuickPass => 1,
            PassDistance::ShortPass => 0,
            PassDistance::LongPass => -1,
            PassDistance::LongBomb => -2,
            PassDistance::HailMary => 0,
        }
    }

    #[staticmethod]
    fn get_casualty_effect(casualty_type: CasualtyType) -> CasualtyEffect {
        match casualty_type {
            CasualtyType::BadlyHurt => CasualtyEffect::None,
            CasualtyType::BrokenRibs => CasualtyEffect::Mng,
            CasualtyType::GroinStrain => CasualtyEffect::Mng,
            CasualtyType::GougedEye => CasualtyEffect::Mng,
            CasualtyType::BrokenJaw => CasualtyEffect::Mng,
            CasualtyType::FracturedArm => CasualtyEffect::Mng,
            CasualtyType::FracturedLeg => CasualtyEffect::Mng,
            CasualtyType::SmashedHand => CasualtyEffect::Mng,
            CasualtyType::PinchedNerve => CasualtyEffect::Mng,
            CasualtyType::DamagedBack => CasualtyEffect::Niggling,
            CasualtyType::SmashedKnee => CasualtyEffect::Niggling,
            CasualtyType::SmashedHip => CasualtyEffect::Ma,
            CasualtyType::SmashedAnkle => CasualtyEffect::Ma,
            CasualtyType::SeriousConcussion => CasualtyEffect::Av,
            CasualtyType::FracturedSkull => CasualtyEffect::Av,
            CasualtyType::BrokenNeck => CasualtyEffect::Ag,
            CasualtyType::SmashedCollarBone => CasualtyEffect::St,
            CasualtyType::Dead => CasualtyEffect::Dead,
        }
    }

    #[classattr]
    fn miss_next_game() -> Vec<CasualtyEffect> {
        vec![
            CasualtyEffect::Mng,
            CasualtyEffect::Ag,
            CasualtyEffect::Av,
            CasualtyEffect::Ma,
            CasualtyEffect::St,
            CasualtyEffect::Niggling,
        ]
    }

    #[classattr]
    fn immovable_action_types() -> Vec<PlayerActionType> {
        vec![PlayerActionType::Block, PlayerActionType::ThrowBomb]
    }

    #[classattr]
    fn pass_player_actions() -> Vec<PlayerActionType> {
        vec![PlayerActionType::Pass, PlayerActionType::ThrowBomb]
    }
}
