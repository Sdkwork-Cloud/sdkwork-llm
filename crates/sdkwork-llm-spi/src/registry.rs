use crate::{LlmImplementationKind, LlmPluginManifest, LlmSpiError, LlmSpiResult};

#[derive(Debug, Default)]
pub struct MemoryPluginRegistry {
    manifests: Vec<LlmPluginManifest>,
}

impl MemoryPluginRegistry {
    pub fn register(&mut self, manifest: LlmPluginManifest) -> LlmSpiResult<()> {
        manifest.validate()?;

        if self
            .manifests
            .iter()
            .any(|existing| existing.plugin_id == manifest.plugin_id)
        {
            return Err(LlmSpiError::DuplicatePluginId(manifest.plugin_id));
        }

        self.manifests.push(manifest);
        Ok(())
    }

    pub fn get(&self, plugin_id: &str) -> Option<&LlmPluginManifest> {
        self.manifests
            .iter()
            .find(|manifest| manifest.plugin_id == plugin_id)
    }

    pub fn plugins_for_implementation(
        &self,
        implementation_kind: LlmImplementationKind,
    ) -> Vec<&LlmPluginManifest> {
        self.manifests
            .iter()
            .filter(|manifest| manifest.implementation_kinds.contains(&implementation_kind))
            .collect()
    }

    pub fn validate_required_ports(
        &self,
        plugin_id: &str,
        required_ports: &[&str],
    ) -> LlmSpiResult<()> {
        let manifest = self
            .get(plugin_id)
            .ok_or_else(|| LlmSpiError::PluginNotFound(plugin_id.to_string()))?;

        for required_port in required_ports {
            let found = manifest
                .port_exports
                .iter()
                .any(|export| export.port == *required_port);

            if !found {
                return Err(LlmSpiError::RequiredPortMissing {
                    plugin_id: plugin_id.to_string(),
                    port: (*required_port).to_string(),
                });
            }
        }

        Ok(())
    }
}
