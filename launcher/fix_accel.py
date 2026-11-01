import re

with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Find the function and replace it
old_func = '''    /// Fabric accelerator settings rows for Horizon-Fabric instances.
    fn accel_settings_rows(&self, cx: &mut Context<Self>, accel: Option<&AccelSettings>) -> Div {
        let t = self.theme;
        let n = t.night;
        let accel = accel.cloned().unwrap_or_default();
        
        div()
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
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(8.))
                    .children([
                        "chunk_io", "worldgen", "lighting", "collision", "meshing", "pathfinding"
                    ].iter().map(|id| {
                        let enabled = *accel.modules.get(*id).unwrap_or(&false);
                        let label = match *id {
                            "chunk_io" => "Chunk & Region I/O",
                            "worldgen" => "World Generation",
                            "lighting" => "Lighting Engine",
                            "collision" => "Collision & Voxel Shapes",
                            "meshing" => "Section Meshing",
                            "pathfinding" => "Pathfinding & Mob AI",
                            _ => id,
                        };
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
                            }
                            cx.notify();
                        }))
                    }).collect()
            )
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
}'''

new_func = '''    /// Fabric accelerator settings rows for Horizon-Fabric instances.
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
        
        div()
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
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(8.))
                    .children(
                        module_ids.iter().map(|id| {
                            let enabled = *accel.modules.get(*id).unwrap_or(&false);
                            let label = module_labels.iter()
                                .find(|(k, _)| *k == *id)
                                .map(|(_, v)| *v)
                                .unwrap_or(id);
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
                                }
                                cx.notify();
                            }))
                        }).collect()
                    )
            )
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
}'''

content = content.replace(old_func, new_func)

with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print('Done')