import re

with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Step 1: Add the is_horizon_fabric check and accel_settings_rows call after the Group row
old1 = '''body = body.child(row(
            "Group",
            self.field(Field::InstanceGroup, "No group", 400., cx)
                .into_any_element(),
        ));

        let mut tiles: Vec<AnyElement> = Vec::new();'''

new1 = '''body = body.child(row(
            "Group",
            self.field(Field::InstanceGroup, "No group", 400., cx)
                .into_any_element(),
        ));
        
        // Fabric accelerator settings for Horizon-Fabric instances
        let is_horizon_fabric = kind == new_instance::NewKind::HorizonFabric 
            || instance.as_ref().map(|i| i.meta.accel.is_some()).unwrap_or(false);
        
        if is_horizon_fabric {
            body = body.child(self.accel_settings_rows(cx, instance.as_ref().and_then(|i| i.meta.accel.as_ref())));
        }

        let mut tiles: Vec<AnyElement> = Vec::new();'''

content = content.replace(old1, new1)

# Step 2: Add the accel_settings_rows method before the closing brace of the first impl Launcher block
old2 = '''        )
        ))
    }
}
'''

new2 = '''        )
        ))

    /// Fabric accelerator settings rows for Horizon-Fabric instances.
    fn accel_settings_rows(&self, cx: &mut Context<Self>, accel: Option<&AccelSettings>) -> Div {
        let t = self.theme;
        let n = t.night;
        let accel = accel.cloned().unwrap_or_default();
        
        let module_ids = ["chunk_io", "worldgen", "lighting", "collision", "meshing", "pathfinding"];
        let module_labels = [
            ("chunk_io", "Chunk & Region I/O"),
            ("worldgen", "World Generation"),
            ("lighting", "Lighting Engine"),
            ("collision", "Collision & Voxel Shapes"),
            ("meshing", "Section Meshing"),
            ("pathfinding", "Pathfinding & Mob AI"),
        ];
        
        let mut div = div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                shell::text(theme::night::TITLE, shell::c(n.ink))
                    .child(tr("Horizon Fabric Accelerator").to_string()),
            )
            .child(
                shell::text(theme::CAPTION).text_color(t.muted).child(
                    tr("Experimental: Horizon's Rust accelerator modules for Fabric mods."),
                ),
            )
            .child(
                shell::action("accel-enabled", if accel.enabled { Action::Raised } else { Action::Quiet }, &self.theme)
                    .child(shell::label(tr("Enable Fabric Accelerator")))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(d) = &mut this.instances.dialog {
                            if let Some(meta) = &mut d.editing {
                                if let Some(accel) = &mut meta.accel {
                                    accel.enabled = !accel.enabled;
                                }
                            }
                        }
                        cx.notify();
                    }),
                )
            .child(
                shell::text(theme::CAPTION).text_color(t.muted).child(
                    tr("Modules:"),
                ),
            );
        
        // Add module toggles
        for id in ["chunk_io", "worldgen", "lighting", "collision", "meshing", "pathfinding"] {
            let enabled = *accel.modules.get(id).unwrap_or(&false);
            let label = match id {
                "chunk_io" => "Chunk & Region I/O",
                "worldgen" => "World Generation",
                "lighting" => "Lighting Engine",
                "collision" => "Collision & Voxel Shapes",
                "meshing" => "Section Meshing",
                "pathfinding" => "Pathfinding & Mob AI",
                _ => id,
            };
            div = div.child(
                shell::action(
                    format!("accel-module-{}", id),
                    if enabled { Action::Raised } else { Action::Quiet },
                    &self.theme,
                )
                .child(shell::label(tr(label)))
                .on_click(cx.listener(move |this, _, cx| {
                    if let Some(d) = &mut this.instances.dialog {
                        if let Some(meta) = &mut d.editing {
                            if let Some(accel) = &mut meta.accel {
                                let new_val = !enabled;
                                accel.modules.insert(id.to_string(), new_val);
                            }
                        }
                        cx.notify();
                    }))
            );
        }
        
        div
            .child(
                shell::text(theme::CAPTION).text_color(t.muted).child(
                    tr("JVM Memory (MiB):"),
                ),
            )
            .child(
                self.field(Field::AccelJvmMemory, "2048", 200., cx)
                    .into_any_element(),
            )
            .child(
                shell::text(theme::CAPTION).text_color(t.muted).child(
                    tr("JVM Arguments:"),
                ),
            )
            .child(
                self.field(Field::AccelJvmArgs, "", 400., cx)
                    .into_any_element(),
            )
    }
}
'''

with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'r', encoding='utf-8') as f:
    content = f.read()

content = content.replace(old1, new1)
content = content.replace(old2, new2)

with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print('Done')