import re

with open('E:\\horison\\horizon\\crates\\horizon-accel\\src\\main\\java\\dev\\horizonlab\\accel\\HorizonAccelMod.java', 'r', encoding='utf-8') as f:
    content = f.read()

# Fix the ModContainer.getPath() calls
content = content.replace(
    'Optional<String> pathOpt = c.getPath("modid");',
    'String pathStr = c.getPath("modid");'
)
content = content.replace(
    'return pathOpt.filter(p -> p.endsWith(".jar")).map(Path::of).orElse(null);',
    'return (pathStr != null && pathStr.endsWith(".jar")) ? Path.of(pathStr) : null;'
)
content = content.replace(
    'Optional<String> pathOpt = c.getPath("modid");',
    'String pathStr = c.getPath("modid");'
)
content = content.replace(
    'return pathOpt.filter(p -> p.endsWith(".jar")).map(Path::of).orElse(null);',
    'return (pathStr != null && pathStr.endsWith(".jar")) ? Path.of(pathStr) : null;'
)

with open('E:\\horison\\horizon\\crates\\horizon-accel\\src\\main\\java\\dev\\horizonlab\\accel\\HorizonAccelMod.java', 'w', encoding='utf-8') as f:
    f.write(content)
print('Done')