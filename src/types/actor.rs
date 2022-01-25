
// struct Vec3<> {

// }

pub trait Actor {
    // fn on_ground(&self) -> bool {
    //     return (*self + 0x1D8) as bool
    // }
    // fn getBodyRot(&self) -> Vec2<f32>;
    // fn getAABB(&self) -> AABB<f32>;
    // fn isRidingMob(&self, actor: &dyn Actor) -> bool;
    
    // fn getFlightSpeed(&self) -> f32;
    // fn setFlightSpeed(&self, speed: f32);

    // fn isFlying(&self) -> bool;
    // fn setFlyState(&self, state: bool);

    // fn getMotion(&self) -> Vec3<f32>;
    // fn setMotion(&self, metion: Vec3<f32>);

    // fn setPos(&mut self, pos: Vec3<f32>);

    // fn _getDimensionId(&self) -> &i32;

    // fn getRuntimeID(&self) -> &i64;
    // fn getXboxGamertag(&self) -> str;

    // fn getRegionConst(&self) -> BlockSource;
    // fn getDimension(&self) -> Dimension;
    // fn getLevel(&self) -> Level;

    // fn isPassenger(&self, actor: &dyn Actor) -> bool;
    // fn getFirstPassenger(&self) -> dyn Actor;

    // fn isPlayerType(&self) -> bool;
    // fn isNotMob(&self) -> bool;
    // fn getEntityType(&self) -> &EntityType;

    // fn outOfWorld(&self) -> bool;
    
    // fn _serverInitItemStackIds(&self);
    // fn _doInitialMove(&self);

    // fn reset(&self);
    // fn getOnDeathExperience(&self) -> &i64;
    // fn getOwnerEntityType(&self) -> &u8;
    // fn remove(&self);
    // fn setPos(&self, pos: Vec3<f32>);
    // fn getPos(&self) -> Vec3<f32>;
    // fn getPosOld(&self) -> Vec3<f32>;
    // // fn getAttachPos(&self, pos: ActorLocation, f32) -> Vec3<f32>;
    // fn getFiringPos(&self) -> Vec3<f32>;
    // fn set_rot(&mut self, rot: Vec2<f32>);
    // fn move_to_proxy(&mut self, movement_proxy: IActorMovementProxy, pos: Vec3<f32>);
    // fn move_to(&mut self, pos: Vec3<f32>);
    // // fn geti32erpolatedRidingPosition(&self, f32) -> Vec3<f32>;
    // // fn geti32erpolatedBodyRot(&self, f32) -> f32;
    // // fn getYawSpeedInDegreesPerSecond(&self) -> f32;
    // // fn geti32erpolatedWalkAnimSpeed(&self, f32) -> f32;
    // fn getWorldPosition(&self) -> Vec3<f32>;
    // fn updateEntityInside(&self);
    // fn updateEntityInside(&self, aabb: AABB<f32>);
    // fn isFireImmune(&self) -> bool;
    // fn blockedByShield(&self, actor: &dyn ActorDamageSource, actor: &dyn Actor);
    // fn teleportTo(&self, pos: Vec3<f32>, bool, i32, i32);
    // fn tryTeleportTo(&self, Vec3<f32>, bool, bool, i32, i32);
    // fn chorusFruitTeleport(&self, Vec3<f32>);
    // fn lerpMotion(&self, Vec3<f32>);
    // fn tryCreateAddActorPacket(&self) -> i64;
    // fn normalTick(&self);
    // fn baseTick(&self);
    // fn rideTick(&self);
    // fn positionPassenger(&self, actor: &dyn Actor);
    // fn startRiding(&self, actor: &dyn Actor) -> bool;
    // fn addPassenger(&self, actor: &dyn Actor);
    // fn flagPassengerToRemove(&self, actor: &dyn Actor);
    // fn i32ersects(&self, Vec3<f32>, Vec3<f32>) -> bool;
    // fn isInWall(&self) -> bool;
    // fn isInvisible(&self) -> bool;
    // fn canShowNameTag(&self) -> bool;
    // fn setNameTagVisible(&self, visibility: bool);
    // fn getNameTag(&self) -> str;
    // fn getFormattedNameTag(&self) -> str;
    // fn filterFormattedNameTag(&self, prof_ctx: &UIProfanityContext);
    // fn setNameTag(&self, name_tag: str);
    // fn setScoreTag(&self, score_tag: str);
    // fn getScoreTag(&self) -> str;
    // fn isInWater(&self) -> bool;
    // fn hasEnteredWater(&self) -> bool;
    // fn isInLava(&self) -> bool;
    // fn isUnderLiquid(&self, MaterialType) -> bool;
    // fn isOverWater(&self) -> bool;
    // fn setBlockMovementSlowdownMultiplier(&self, Vec3<f32>);
    // fn resetBlockMovementSlowdownMultiplier(&self);
    // fn getShadowHeightOffs(&self) -> f32;
    // fn getShadowRadiusOffs(&self) -> f32;
    // fn getHeadLookVector(&self, f32) -> Vec3<f32>;
    // fn canSee(&self, Vec3<f32>) -> bool;
    // // fn canSee(&self, actor: &dyn Actor) -> bool;
    // fn isSkyLit(&self, f32) -> bool;
    // fn getBrightness(&self, f32) -> f32;
    // fn isImmobile(&self) -> bool;
    // fn isSilent(&self) -> bool;
    // fn isSleeping(&self) -> bool;
    // fn setSleeping(&self, sleeping: bool);
    // fn setSneaking(&self, sneaking: bool);
    // fn isAlive(&self) -> bool;
    // fn isOnFire(&self) -> bool;
    // fn isOnHotBlock(&self) -> bool;
    // fn isAffectedByWaterBottle(&self) -> bool;
    // fn canAttack(&self, actor: &dyn Actor, can_attack: bool) -> bool;
    // fn setTarget(&self, actor: &dyn Actor);
    // fn isValidTarget(&self, actor: &dyn Actor) -> bool;
    // fn attack(&self, actor: &dyn Actor, cause: ActorDamageCause) -> bool;
    // fn performRangedAttack(&self, actor: &dyn Actor, f32);
    // fn getEquipmentCount(&self) -> i64;
    // fn setOwner(&self, auid: &ActorUniqueID);
    // fn setSitting(&self, bool);
    // fn getInventorySize(&self) -> i64;
    // fn getEquipSlots(&self) -> i64;
    // fn getChestSlots(&self) -> i64;
    // fn setStanding(&self, bool);
    // fn canPowerJump(&self) -> bool;
    // fn setCanPowerJump(&self, bool);
    // fn isJumping(&self) -> bool;
    // fn isEnchanted(&self) -> bool;
    // fn shouldRender(&self) -> bool;
    // fn playAmbientSound(&self);
    // fn getAmbientSound(&self) -> LevelSoundEvent;
    // fn isInvulnerableTo(&self, dmg_src: &ActorDamageSource) -> bool;
    // fn getBlockDamageCause(&self, &Block) -> ActorDamageCause;
    // fn actuallyHurt(&self, i32, &ActorDamageSource, bool);
    // fn animateHurt(&self);
    // fn doFireHurt(&self, i32) -> bool;
    // fn feed(&self, i32);
    // fn handleEntityEvent(&self, ActorEvent, i32);
    // fn getPickRadius(&self) -> f32;
    // fn getActorRendererId(&self) -> HashedString;
    // fn spawnAtLocation(&self, &ItemStack, f32) -> ItemActor;
    // fn spawnAtLocation(&self, class &Block*, i32, f32) -> ItemActor;
    // fn spawnAtLocation(&self, class &Block*, i32) -> ItemActor;
    // fn spawnAtLocation(&self, i32, i32, f32) -> ItemActor;
    // fn spawnAtLocation(&self, i32, i32) -> ItemActor;
    // fn despawn(&self);
    // fn killed(&self, actor: &dyn Actor);
    // fn setArmorSlot(&self, i32, &ItemStack);
    // fn getArmor(&self, i32) -> ItemStack;
    // fn getAllArmor(&self) -> Vec<&ItemStack>;
    // fn getModelScale(&self) -> f32;
    // fn getEquippedSlot(&self, i32) -> ItemStack;
    // fn getCarriedItem(&self) -> ItemStack;
    // fn setCarriedItem(&self, &ItemStack);
    // fn setOffHandSlot(&self, &ItemStack);
    // fn getEquippedTotem(&self) -> ItemStack;
    // fn consumeTotem(&self);
    // fn getEntityTypeId(&self) -> u32;
    // fn getSourceUniqueID(&self) -> *const ActorUniqueID;
    // fn thawFreezeEffect(&self);
    // fn isWearingLeatherArmor(&self) -> bool;
    // fn getLiquidAABB(&self, MaterialType) -> AABB<f32>;
    // fn handleInsidePortal(&self, Vec3<i32>);
    // fn getPortalCooldown(&self) -> i64;
    // fn getDimensionId(&self) -> i64;
    // fn changeDimension(&self, i64);
    // fn checkFallDamage(&self, f32, bool);
    // fn causeFallDamage(&self, f32, f32, struct ActorDamageSource &);
    // fn handleFallDistanceOnServer(&self, f32, f32, bool);
    // fn playSynchronizedSound(&self, LevelSoundEvent, Vec3<f32>, i32, bool);
    // fn playSynchronizedSound(&self, LevelSoundEvent, Vec3<f32>, Block*, bool);
    // fn canAddPassenger(&self, actor: &dyn Actor) -> bool;
    // fn tickLeash(&self);
    // fn sendMotionPacketIfNeeded(&self);
    // fn stopRiding(&self, bool, bool, bool);
    // fn startSwimming(&self);
    // fn stopSwimming(&self);
    // fn getCommandPermissionLevel(&self) -> CommandPermissionLevel;
    // fn isClientSide(&self) -> bool;
    // fn getMutableAttribute(&self, Attribute*) -> AttributeInstance;
    // fn getAttribute(&self, AttributeInstance*) -> Attribute;
    // fn heal(&self, i32);
    // fn isInvertedHealAndHarm(&self) -> bool;
    // fn canBeAffected(&self, struct MobEffectInstance*) -> bool;
    // fn canBeAffected(&self, i32) -> bool;
    // fn canBeAffectedByArrow(&self, MobEffectInstance*) -> bool;
    // fn getAnimationComponent(&self) -> AnimationComponent;
    // fn openContainerComponent(&self, Player*);
    // fn swing(&self);
    // fn useItem(&self, class ItemStackBase*, enum ItemUseMethod, bool);
    // fn getMapDecorationRotation(&self) -> f32;
    // fn getPassengerYRotation(&self, actor: &dyn Actor) -> f32;
    // fn getYHeadRot(&self) -> f32;
    // fn isWorldBuilder(&self) -> bool;
    // fn isInCreativeMode(&self) -> bool;
    // fn isAdventure(&self) -> bool;
    // fn addItem(&self, ItemStack*) -> bool;
    // fn drop(&self, ItemStack*) -> bool;
    // fn geti32eraction(&self, Player*, struct Actori32eraction*, Vec3<f32>) -> bool;
    // fn setSize(&self, f32, f32);
    // fn wobble(&self);
    // fn wasHurt(&self) -> bool;
    // fn startSpinAttack(&self);
    // fn setDamageNearbyMobs(&self, bool);
    // fn reloadLootTable(&self, struct EquipmentTableDefinition*);
    // fn reloadLootTable(&self);
    // fn kill(&self);
    // fn die(&self, ActorDamageSource*);
    // fn shouldDropDeathLoot(&self) -> bool;
    // fn shouldTick(&self) -> bool;
    // fn getNextStep(&self, f32) -> f32;
    // fn getLootTable(&self) -> LootTable;
    // fn shouldTryMakeStepSound(&self) -> bool;
    // fn markHurt(&self);
    // fn checkInsideBlocks(&self, f32);
    // fn pushOutOfBlocks(&self, Vec3<f32>);
    // fn updateWaterState(&self);
    // fn doWaterSplashEffect(&self);
    // fn spawnTrailBubbles(&self);
    // fn updateInsideBlock(&self);
}