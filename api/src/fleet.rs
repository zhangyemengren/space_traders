use crate::common::{get, Destination, Response, SuccessVec, Success, get_token};
use reqwest::{StatusCode, Url};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Role {
    FABRICATOR,
    HARVESTER,
    HAULER,
    INTERCEPTOR,
    EXCAVATOR,
    TRANSPORT,
    REPAIR,
    SURVEYOR,
    COMMAND,
    CARRIER,
    PATROL,
    SATELLITE,
    EXPLORER,
    REFINERY,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Registration {
    name: String,
    #[serde(rename = "factionSymbol")]
    faction_symbol: String,
    role: Role,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Route {
    destination: Destination,
    departure: Destination,
    #[serde(rename = "departureTime")]
    departure_time: String,
    arrival: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Status {
    #[serde(rename = "IN_TRANSIT")]
    InTransit,
    #[serde(rename = "IN_ORBIT")]
    InOrbit,
    DOCKED,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub enum FlightMode {
    DRIFT,
    STEALTH,
    #[default]
    CRUISE,
    BURN,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Nav {
    #[serde(rename = "systemSymbol")]
    system_symbol: String,
    #[serde(rename = "waypointSymbol")]
    waypoint_symbol: String,
    status: Status,
    #[serde(rename = "flightMode")]
    flight_mode: String,
    route: Route,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Requirements {
    #[serde(default)]
    power: u32,
    #[serde(default)]
    crew: u32,
    #[serde(default)]
    slots: u32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub enum Rotation {
    #[default]
    STRICT,
    RELAXED,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Crew {
    current: u32,
    required: u32,
    capacity: u32,
    rotation: String,
    morale: u32,
    wages: u32,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum FrameSymbol {
    #[serde(rename = "FRAME_PROBE")]
    FrameProbe,
    #[serde(rename = "FRAME_DRONE")]
    FrameDrone,
    #[serde(rename = "FRAME_INTERCEPTOR")]
    FrameInterceptor,
    #[serde(rename = "FRAME_RACER")]
    FrameRacer,
    #[serde(rename = "FRAME_FIGHTER")]
    FrameFighter,
    #[serde(rename = "FRAME_FRIGATE")]
    FrameFrigate,
    #[serde(rename = "FRAME_SHUTTLE")]
    FrameShuttle,
    #[serde(rename = "FRAME_EXPLORER")]
    FrameExplorer,
    #[serde(rename = "FRAME_MINER")]
    FrameMiner,
    #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
    FrameLightFreighter,
    #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
    FrameHeavyFreighter,
    #[serde(rename = "FRAME_TRANSPORT")]
    FrameTransport,
    #[serde(rename = "FRAME_DESTROYER")]
    FrameDestroyer,
    #[serde(rename = "FRAME_CRUISER")]
    FrameCruiser,
    #[serde(rename = "FRAME_CARRIER")]
    FrameCarrier,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Frame {
    symbol: FrameSymbol,
    name: String,
    description: String,
    condition: u32,
    #[serde(rename = "moduleSlots")]
    module_slots: u32,
    #[serde(rename = "mountingPoints")]
    mounting_points: u32,
    #[serde(rename = "fuelCapacity")]
    fuel_capacity: u32,
    requirements: Requirements,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ReactorSymbol {
    #[serde(rename = "REACTOR_SOLAR_I")]
    ReactorSolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    ReactorFusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    ReactorFissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ReactorChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    ReactorAntimatterI,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Reactor {
    symbol: ReactorSymbol,
    name: String,
    description: String,
    condition: u32,
    #[serde(rename = "powerOutput")]
    power_output: u32,
    requirements: Requirements,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum EngineSymbol {
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    EngineImpulseDriveI,
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_II")]
    EngineImpulseDriveII,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    EngineIonDriveII,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    EngineHyperDriveI,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Engine {
    symbol: EngineSymbol,
    name: String,
    description: String,
    condition: u32,
    speed: u32,
    requirements: Requirements,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ModulesSymbol {
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    ModuleMineralProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    ModuleCargoHoldI,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    ModuleCrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    ModuleEnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    ModulePassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    ModuleMicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    ModuleOreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    ModuleFuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ModuleScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    ModuleJumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    ModuleJumpDriveII,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    ModuleJumpDriveIII,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    ModuleWarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    ModuleWarpDriveII,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    ModuleWarpDriveIII,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ModuleShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ModuleShieldGeneratorII,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Modules {
    symbol: ModulesSymbol,
    #[serde(default)]
    capacity: u32,
    #[serde(default)]
    range: u32,
    name: String,
    description: String,
    requirements: Requirements,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum MountsSymbol {
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    MountGasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    MountGasSiphonII,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    MountGasSiphonIII,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    MountSurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    MountSurveyorII,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    MountSurveyorIII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    MountSensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    MountSensorArrayII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    MountSensorArrayIII,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MountMiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MountMiningLaserII,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MountMiningLaserIII,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    MountLaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MountMissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    MountTurretI,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Deposits {
    #[serde(rename = "QUARTZ_SAND")]
    QuartzSand,
    #[serde(rename = "SILICON_CRYSTALS")]
    SiliconCrystals,
    #[serde(rename = "PRECIOUS_STONES")]
    PreciousStones,
    #[serde(rename = "ICE_WATER")]
    IceWater,
    #[serde(rename = "AMMONIA_ICE")]
    AmmoniaIce,
    #[serde(rename = "IRON_ORE")]
    IronOre,
    #[serde(rename = "COPPER_ORE")]
    CopperOre,
    #[serde(rename = "SILVER_ORE")]
    SilverOre,
    #[serde(rename = "ALUMINUM_ORE")]
    AluminumOre,
    #[serde(rename = "GOLD_ORE")]
    GoldOre,
    #[serde(rename = "PLATINUM_ORE")]
    PlatinumOre,
    #[serde(rename = "DIAMONDS")]
    Diamonds,
    #[serde(rename = "URANITE_ORE")]
    UraniteOre,
    #[serde(rename = "MERITIUM_ORE")]
    MeritiumOre,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Mounts {
    symbol: MountsSymbol,
    name: String,
    description: String,
    strength: u32,
    #[serde(default)]
    deposits: Vec<Deposits>,
    requirements: Requirements,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Inventory {
    symbol: String,
    name: String,
    description: String,
    units: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Cargo {
    capacity: u32,
    units: u32,
    inventory: Vec<Inventory>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Consumed {
    amount: u32,
    #[serde(rename = "timestamp")]
    time_stamp: String,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Fuel {
    current: u32,
    capacity: u32,
    consumed: Consumed,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Ship {
    symbol: String,
    registration: Registration,
    nav: Nav,
    crew: Crew,
    frame: Frame,
    reactor: Reactor,
    engine: Engine,
    modules: Vec<Modules>,
    mounts: Vec<Mounts>,
    cargo: Cargo,
    fuel: Fuel,
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CoolDown{
    #[serde(rename = "shipSymbol")]
    ship_symbol: String,
    #[serde(rename = "totalSeconds")]
    total_seconds: u32,
    #[serde(rename = "remainingSeconds")]
    remaining_seconds: u32,
    expiration: String,
}

/// 列出你所有的船
pub async fn list_ships(
    page: &str,
) -> Result<Response<SuccessVec<Ship>>, Box<dyn std::error::Error>> {
    let mut url = Url::parse("https://api.spacetraders.io/v2/my/ships").expect("url parse error");
    url.query_pairs_mut()
        .append_pair("limit", "20")
        .append_pair("page", page);
    get(url.as_str()).await
}

/// 获取我的船的详细信息
pub async fn get_ship_detail(ship: &str) -> Result<Response<Success<Ship>>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://api.spacetraders.io/v2/my/ships/{}", ship)).expect("url parse error");
    get(url.as_str()).await
}

/// 获取你船上的货物信息
pub async fn get_ship_cargo(ship: &str) -> Result<Response<Success<Cargo>>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://api.spacetraders.io/v2/my/ships/{}/cargo", ship)).expect("url parse error");
    get(url.as_str()).await
}
/// 检索您的飞船反应堆冷却时间的详细信息。激活跳跃引擎、扫描或提取资源等某些操作会加重反应堆的负担并导致冷却时间。
///
/// 在您的冷却时间到期之前，您的飞船无法执行其他操作。冷却时间的长短与相关模块或坐骑的功耗有关。
///
/// 当飞船没有冷却时间时，响应返回 204 状态代码（无内容）。
pub async fn get_ship_cooldown(ship: &str) -> Result<Response<Success<CoolDown>>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("https://api.spacetraders.io/v2/my/ships/{}/cooldown", ship)).expect("url parse error");
    let c = reqwest::Client::new();
    let res = c
        .get(url)
        .header("Authorization","Bearer ".to_string() + get_token().as_str())
        .send()
        .await?;
    let status = res.status();
    match status {
        StatusCode::OK => {
            let s = res.json::<Success<CoolDown>>().await?;
            Ok(Response::Success(s))
        }
        StatusCode::NO_CONTENT => {
            Ok(Response::NoData)
        }
        _ => {
            let e = res.text().await?;
            println!("{}", e);
            Err("获取飞船冷却时间失败".into())
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_ships() {
        let res = list_ships("1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
    #[tokio::test]
    async fn test_get_ship_detail() {
        let res = get_ship_detail("ZHANGYEMENGREN-1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
    #[tokio::test]
    async fn test_get_ship_cargo() {
        let res = get_ship_cargo("ZHANGYEMENGREN-1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
    #[tokio::test]
    async fn test_get_ship_cooldown() {
        let res = get_ship_cooldown("ZHANGYEMENGREN-1").await.unwrap();
        println!("{:#?}", res);
        assert!(true);
    }
}
