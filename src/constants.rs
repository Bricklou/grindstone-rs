/// URL of the Minecraft version manifest.
pub const MC_VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

/// Base URL for Minecraft libraries.
pub const MC_LIBRARIES_BASE_URL: &str = "https://libraries.minecraft.net";

/// URL to the manifest to obtains Java JRE
pub const JAVA_JRE_MANIFEST_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";

/// Base URL for Minecraft assets.
pub const MC_ASSETS_BASE_URL: &str = "https://resources.download.minecraft.net";

#[cfg(windows)]
pub const MC_MS_STORE_IDENTIFIANT: &str = "Microsoft.4297127D64EC6_8wekyb3d8bbwe";

/// Maximum files downloaded at the same time
pub const MAX_PARALLEL_DOWNLOAD: usize = 50;
