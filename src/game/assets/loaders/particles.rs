use std::fmt::Debug;

use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    utils::HashMap,
};
use bevy_hanabi::*;
use ron::de::from_bytes;
use serde::Deserialize;

use crate::utils::get_file_name;

use super::ron::RonLoaderError;

// ------------------- Value for properties
#[derive(Deserialize, Clone, Debug)]
enum V {
    F32(f32),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    Vec4(f32, f32, f32, f32),
}

impl V {
    const F32_ZERO: V = V::F32(0.);
    const V2_ZERO: V = V::Vec2(0., 0.);
    const V3_ZERO: V = V::Vec3(0., 0., 0.);
    const V4_ZERO: V = V::Vec4(0., 0., 0., 0.);
}

impl Into<f32> for V {
    fn into(self) -> f32 {
        match self {
            V::F32(value) => value,
            _ => 0.,
        }
    }
}

impl Into<Vec2> for V {
    fn into(self) -> Vec2 {
        match self {
            V::Vec2(x, y) => Vec2::new(x, y),
            _ => Vec2::ZERO,
        }
    }
}

impl Into<Vec3> for V {
    fn into(self) -> Vec3 {
        match self {
            V::Vec3(x, y, z) => Vec3::new(x, y, z),
            _ => Vec3::ZERO,
        }
    }
}

impl Into<Vec4> for V {
    fn into(self) -> Vec4 {
        match self {
            V::Vec4(x, y, z, w) => Vec4::new(x, y, z, w),
            _ => Vec4::ZERO,
        }
    }
}

impl Into<Value> for V {
    fn into(self) -> Value {
        match self {
            V::F32(value) => Value::Scalar(ScalarValue::Float(value)),
            V::Vec2(x, y) => Value::Vector(VectorValue::new_vec2(Vec2::new(x, y))),
            V::Vec3(x, y, z) => Value::Vector(VectorValue::new_vec3(Vec3::new(x, y, z))),
            V::Vec4(x, y, z, w) => Value::Vector(VectorValue::new_vec4(Vec4::new(x, y, z, w))),
        }
    }
}

impl Default for V {
    fn default() -> Self {
        Self::F32(0.)
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
enum FileValue<T> {
    Literal(T),
    Rand(T, T), // start, end
    Property(String),
}

impl FileValue<f32> {
    fn into_expr(
        self,
        properties: &HashMap<String, WriterExpr>,
        writer: &ExprWriter,
    ) -> ExprHandle {
        match self {
            FileValue::Literal(value) => writer.lit(value).expr(),
            FileValue::Rand(start, end) => {
                (writer.rand(ScalarType::Float) * writer.lit(end - start) + writer.lit(start))
                    .expr()
            }
            FileValue::Property(name) => properties.get(&name).unwrap().clone().expr(),
        }
    }
}

impl FileValue<Vec2> {
    fn into_expr(
        self,
        properties: &HashMap<String, WriterExpr>,
        writer: &ExprWriter,
    ) -> ExprHandle {
        match self {
            FileValue::Literal(value) => writer.lit(value).expr(),
            FileValue::Rand(start, end) => {
                (writer.rand(ScalarType::Float) * writer.lit(end - start) + writer.lit(start))
                    .expr()
            }
            FileValue::Property(name) => properties.get(&name).unwrap().clone().expr(),
        }
    }
}

impl FileValue<Vec3> {
    fn into_expr(
        self,
        properties: &HashMap<String, WriterExpr>,
        writer: &ExprWriter,
    ) -> ExprHandle {
        match self {
            FileValue::Literal(value) => writer.lit(value).expr(),
            FileValue::Rand(start, end) => {
                (writer.rand(ScalarType::Float) * writer.lit(end - start) + writer.lit(start))
                    .expr()
            }
            FileValue::Property(name) => properties.get(&name).unwrap().clone().expr(),
        }
    }
}

impl FileValue<Vec4> {
    fn into_expr(
        self,
        properties: &HashMap<String, WriterExpr>,
        writer: &ExprWriter,
    ) -> ExprHandle {
        match self {
            FileValue::Literal(value) => writer.lit(value).expr(),
            FileValue::Rand(start, end) => {
                (writer.rand(ScalarType::Float) * writer.lit(end - start) + writer.lit(start))
                    .expr()
            }
            FileValue::Property(name) => properties.get(&name).unwrap().clone().expr(),
        }
    }
}

impl Default for FileValue<f32> {
    fn default() -> Self {
        Self::Literal(0.)
    }
}

impl Default for FileValue<Vec2> {
    fn default() -> Self {
        Self::Literal(Vec2::ZERO)
    }
}

impl Default for FileValue<Vec3> {
    fn default() -> Self {
        Self::Literal(Vec3::ZERO)
    }
}

impl Default for FileValue<Vec4> {
    fn default() -> Self {
        Self::Literal(Vec4::ZERO)
    }
}

// ------------------- Spawner
#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct SpawnerOnceConf {
    count: f32,
    spawn_immediately: bool,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct SpawnerRateConf {
    particles_per_seconds: f32,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
pub struct SpawnerBurstConf {
    count: f32,
    period: f32,
}

#[derive(Deserialize, Clone, Debug)]
enum SpawnerConf {
    Once(SpawnerOnceConf),
    Rate(SpawnerRateConf),
    Burst(SpawnerBurstConf),
}

impl Default for SpawnerConf {
    fn default() -> Self {
        Self::Once(SpawnerOnceConf {
            count: 1.,
            spawn_immediately: false,
        })
    }
}

// ------------------- Postion modifiers

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct PositionConeConf {
    /// f32
    base_radius: FileValue<f32>,
    /// f32
    height: FileValue<f32>,
    /// f32
    top_radius: FileValue<f32>,
    dimension: ShapeDimension,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
struct PositionCircleConf {
    /// Vec3
    center: FileValue<Vec3>,
    /// f32
    radius: FileValue<f32>,
    /// Vec3
    axis: FileValue<Vec3>,
    dimension: ShapeDimension,
}

impl Default for PositionCircleConf {
    fn default() -> Self {
        Self {
            center: FileValue::default(),
            radius: FileValue::default(),
            axis: FileValue::Literal(Vec3::Z),
            dimension: ShapeDimension::Surface,
        }
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct PositionSphereConf {
    /// Vec3
    center: FileValue<Vec3>,
    /// f32
    radius: FileValue<f32>,
    dimension: ShapeDimension,
}

#[derive(Deserialize, Clone, Debug)]
enum PositionModifierConf {
    Circle(PositionCircleConf),
    Sphere(PositionSphereConf),
    Attribute(FileValue<Vec3>),
    Cone(PositionConeConf),
}

impl Default for PositionModifierConf {
    fn default() -> Self {
        Self::Attribute(FileValue::default())
    }
}

// ------------------- Velocity modifiers
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
struct VelocityCircleConf {
    /// Vec3
    center: FileValue<Vec3>,
    /// Vec3
    axis: FileValue<Vec3>,
    /// f32
    speed: FileValue<f32>,
}

impl Default for VelocityCircleConf {
    fn default() -> Self {
        Self {
            center: FileValue::default(),
            axis: FileValue::Literal(Vec3::Z),
            speed: FileValue::Literal(1.),
        }
    }
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct VelocitySphereConf {
    /// Vec3
    center: FileValue<Vec3>,
    /// f32
    speed: FileValue<f32>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct VelocityTangentConf {
    origin: FileValue<Vec3>,
    axis: FileValue<Vec3>,
    speed: FileValue<f32>,
}

#[derive(Deserialize, Clone, Debug)]
enum VelocityModifierConf {
    Circle(VelocityCircleConf),
    Sphere(VelocitySphereConf),
    Tangent(VelocityTangentConf),
    Attribute(FileValue<Vec3>),
}

impl Default for VelocityModifierConf {
    fn default() -> Self {
        Self::Circle(VelocityCircleConf::default())
    }
}

// ------------------- Accel modifiers
#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct RadialAccelConf {
    origin: FileValue<Vec3>,
    accel: FileValue<f32>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct TangentAccelConf {
    origin: FileValue<Vec3>,
    axis: FileValue<Vec3>,
    accel: FileValue<f32>,
}

#[derive(Deserialize, Clone, Debug)]
enum AccelModifierConf {
    Attribute(FileValue<Vec3>),
    Radial(RadialAccelConf),
    Tangent(TangentAccelConf),
}

impl Default for AccelModifierConf {
    fn default() -> Self {
        Self::Attribute(FileValue::default())
    }
}

// ------------------- Render modifiers
#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct ColorOverLifetimeModifierConf {
    gradient: Vec<(f32, Vec4)>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct SizeOverLifetimeModifierConf {
    gradient: Vec<(f32, Vec2)>,
    screen_space_size: bool,
}

// ------------------- Modifiers groups
#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct InitModifiers {
    position: PositionModifierConf,
    velocity: VelocityModifierConf,
    /// SetAttributeModifier(Attribute::LIFETIME, lifetime)
    lifetime: FileValue<f32>,
    /// SetAttributeModifier(Attribute::SIZE, size)
    age: FileValue<f32>,
    /// SetAttributeModifier(Attribute::SIZE, size)
    size: FileValue<f32>,
    /// SetAttributeModifier(Attribute::SIZE2, size2)
    size2: FileValue<Vec2>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct UpdateModifiers {
    accel: AccelModifierConf,
    linear_drag: FileValue<f32>,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct RenderModifiers {
    // particle_texture: ,
    /// SetColorModifier
    color: Vec4,
    /// ColorOverLifetimeModifier
    color_over_lifetime: ColorOverLifetimeModifierConf,
    /// SetSizeModifier
    size: Vec2,
    /// SizeOverLifetimeModifier
    size_over_lifetime: SizeOverLifetimeModifierConf,
    // orient: ,
    // flipbook: ,
    // screen_space_size: ,
    // round: ,
}

#[derive(Deserialize, Clone, Debug, Default)]
#[serde(default)]
struct ParticleFile {
    capacities: Vec<u32>,
    spawner: SpawnerConf,
    properties: HashMap<String, V>,
    init: InitModifiers,
    update: UpdateModifiers,
    render: RenderModifiers,
}

pub struct HanabiEffectLoader;

impl AssetLoader for HanabiEffectLoader {
    type Asset = EffectAsset;
    type Settings = ();
    type Error = RonLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let particle_file = from_bytes::<ParticleFile>(&bytes)?;
        info!(
            "Load {} particle file.",
            get_file_name(&load_context.path().to_path_buf())
        );

        // create the particle effect
        let writer = ExprWriter::new();

        // create properties
        let mut properties = HashMap::new();
        for (name, value) in particle_file.properties {
            let prop_handle = writer.add_property(name.clone(), value.into());
            let prop_writer = writer.prop(prop_handle);
            properties.insert(name.clone(), prop_writer);
        }

        // init over lifetime modifiers
        let mut color_gradient = Gradient::new();
        for (ratio, color) in particle_file
            .render
            .color_over_lifetime
            .gradient
            .clone()
            .into_iter()
        {
            color_gradient.add_key(ratio, color);
        }
        let color_over_lifetime_modifier = ColorOverLifetimeModifier {
            gradient: color_gradient,
        };

        let mut size_gradient = Gradient::new();
        for (ratio, size) in particle_file
            .render
            .size_over_lifetime
            .gradient
            .clone()
            .into_iter()
        {
            size_gradient.add_key(ratio, size);
        }
        let size_over_lifetime_modifier = SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: particle_file.render.size_over_lifetime.screen_space_size,
        };

        // initialise position, velocity and lifetime attributes
        let init_pos: Box<dyn Modifier> = match particle_file.init.position {
            PositionModifierConf::Sphere(conf) => Box::new(SetPositionSphereModifier {
                center: conf.center.into_expr(&properties, &writer),
                radius: conf.radius.into_expr(&properties, &writer),
                dimension: conf.dimension,
            }),
            PositionModifierConf::Circle(conf) => Box::new(SetPositionCircleModifier {
                center: conf.center.into_expr(&properties, &writer),
                radius: conf.radius.into_expr(&properties, &writer),
                axis: conf.axis.into_expr(&properties, &writer),
                dimension: conf.dimension,
            }),
            PositionModifierConf::Cone(conf) => Box::new(SetPositionCone3dModifier {
                base_radius: conf.base_radius.into_expr(&properties, &writer),
                height: conf.height.into_expr(&properties, &writer),
                top_radius: conf.top_radius.into_expr(&properties, &writer),
                dimension: conf.dimension,
            }),
            PositionModifierConf::Attribute(value) => Box::new(SetAttributeModifier::new(
                Attribute::POSITION,
                value.into_expr(&properties, &writer),
            )),
        };

        let init_vel: Box<dyn Modifier> = match particle_file.init.velocity {
            VelocityModifierConf::Sphere(conf) => Box::new(SetVelocitySphereModifier {
                center: conf.center.into_expr(&properties, &writer),
                speed: conf.speed.into_expr(&properties, &writer),
            }),
            VelocityModifierConf::Circle(conf) => Box::new(SetVelocityCircleModifier {
                center: conf.center.into_expr(&properties, &writer),
                axis: conf.axis.into_expr(&properties, &writer),
                speed: conf.speed.into_expr(&properties, &writer),
            }),
            VelocityModifierConf::Tangent(conf) => Box::new(SetVelocityTangentModifier {
                origin: conf.origin.into_expr(&properties, &writer),
                axis: conf.axis.into_expr(&properties, &writer),
                speed: conf.speed.into_expr(&properties, &writer),
            }),
            VelocityModifierConf::Attribute(conf) => Box::new(SetAttributeModifier::new(
                Attribute::VELOCITY,
                conf.into_expr(&properties, &writer),
            )),
        };

        let init_lifetime = SetAttributeModifier::new(
            Attribute::LIFETIME,
            particle_file.init.lifetime.into_expr(&properties, &writer),
        );

        let init_age = SetAttributeModifier::new(
            Attribute::AGE,
            particle_file.init.age.into_expr(&properties, &writer),
        );

        // FIXME - clone()
        let init_size = SetAttributeModifier::new(
            Attribute::SIZE,
            particle_file
                .init
                .size
                .clone()
                .into_expr(&properties, &writer),
        );

        let init_size2 = SetAttributeModifier::new(
            Attribute::SIZE2,
            particle_file
                .init
                .size2
                .clone()
                .into_expr(&properties, &writer),
        );

        // update modifiers
        let update_accel: Box<dyn Modifier> = match particle_file.update.accel {
            AccelModifierConf::Attribute(value) => {
                Box::new(AccelModifier::new(value.into_expr(&properties, &writer)))
            }
            AccelModifierConf::Radial(conf) => Box::new(RadialAccelModifier::new(
                conf.origin.into_expr(&properties, &writer),
                conf.accel.into_expr(&properties, &writer),
            )),
            AccelModifierConf::Tangent(conf) => Box::new(TangentAccelModifier::new(
                conf.origin.into_expr(&properties, &writer),
                conf.axis.into_expr(&properties, &writer),
                conf.accel.into_expr(&properties, &writer),
            )),
        };

        let update_linear_drag = LinearDragModifier::new(
            particle_file
                .update
                .linear_drag
                .clone()
                .into_expr(&properties, &writer),
        );

        // init spawner
        let spawner = match particle_file.spawner {
            SpawnerConf::Once(conf) => Spawner::once(conf.count.into(), conf.spawn_immediately),
            SpawnerConf::Rate(conf) => Spawner::rate(conf.particles_per_seconds.into()),
            SpawnerConf::Burst(conf) => Spawner::burst(conf.count.into(), conf.period.into()),
        };

        let mut effect = EffectAsset::new(particle_file.capacities, spawner, writer.finish())
            .with_name(get_file_name(&load_context.path().to_path_buf()))
            .add_modifier(ModifierContext::Init, init_pos)
            .add_modifier(ModifierContext::Init, init_vel)
            .init(init_age)
            .init(init_lifetime)
            .add_modifier(ModifierContext::Update, update_accel);

        if particle_file.init.size != FileValue::default() {
            effect = effect.init(init_size);
        }

        if particle_file.init.size2 != FileValue::default() {
            effect = effect.init(init_size2);
        }

        if particle_file.update.linear_drag != FileValue::default() {
            effect = effect.update(update_linear_drag);
        }

        if particle_file.render.color != Vec4::default() {
            effect = effect.render(SetColorModifier {
                color: particle_file.render.color.into(),
            });
        }

        if particle_file.render.color_over_lifetime.gradient.len() > 0 {
            effect = effect.render(color_over_lifetime_modifier);
        }

        if particle_file.render.size != Vec2::default() {
            effect = effect.render(SetSizeModifier {
                size: particle_file.render.size.into(),
            });
        }

        if particle_file.render.size_over_lifetime.gradient.len() > 0 {
            effect = effect.render(size_over_lifetime_modifier);
        }

        Ok(effect)
    }

    fn extensions(&self) -> &[&str] {
        &["particle.ron"]
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_particle_file() {
        let content = fs::read_to_string(
            "/home/celes/documents/lab/a-game/assets/particles/effects/dash.particle.ron",
        )
        .unwrap();
        match ron::from_str::<ParticleFile>(&content) {
            Err(e) => println!("{e}"),
            Ok(_) => {}
        }
        assert!(!matches!(ron::from_str::<ParticleFile>(&content), Err(_)))
    }
}
