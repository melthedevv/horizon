with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# Step 1: Add the is_horizon_fabric check and accel_settings_rows call after the Group row
old = '''body = body.child(row(
            "Group",
            self.field(Field::InstanceGroup, "No group", 400., cx)
                .into_any_element(),
        ));

        let mut tiles: Vec<AnyElement> = Vec::new();'''

new = '''body = body.child(row(
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

content = content.replace(old, new)

with open('E:\\horison\\horizon\\launcher\\src\\instances_page.rs', 'w', encoding='utf-8') as f:
    f.write(content)
print('Step 1 done')