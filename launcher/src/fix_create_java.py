import re

with open('E:\\horison\\horizon\\launcher\\src\\new_instance.rs', 'r', encoding='utf-8') as f:
    content = f.read()

old = """    let loader = (choice.kind == NewKind::Modded)
        .then(|| loaders::choose(choice.loader.id(), minecraft, None))
        .transpose()?;
    let made = instances::create_java(name, minecraft)?;
    match loader {
        Some(loader) => instances::edit(&made.id, |m| {
            m.java.get_or_insert_with(Default::default).loader = Some(loader)
        }),
        None => Ok(made),
    }"""

new = """    let loader = match choice.kind {
            NewKind::Modded => loaders::choose(choice.loader.id(), minecraft, None).map(Some)?,
            NewKind::HorizonFabric => {
                // HorizonFabric is pinned to 26.3 with Fabric loader
                loaders::choose(loaders::LoaderId::Fabric, "26.3", None).map(Some)?
            }
            _ => None,
        };
    let made = instances::create_java(name, minecraft)?;
    match loader {
        Some(loader) => instances::edit(&made.id, |m| {
            m.java.get_or_insert_with(Default::default).loader = Some(loader)
        }),
        None => Ok(made),
    }"""

content = content.replace(old, new)

with open('E:\\horison\\horizon\\launcher\\src\\new_instance.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("Done")