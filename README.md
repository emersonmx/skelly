# skelly

A simple scaffold tool written in Rust.

## Usage

```
Usage: skelly [OPTIONS] <SKELETON_PATH> [INPUTS]...

skelly ~/path/to/skeleton input1=one input2=two inputN=N
```

## Template creation

```sh
# Create a template directory
mkdir my_template
cd my_template

# Create a skelly.toml
cat > skelly.toml << EOF
[[inputs]]
name = "full_name"
EOF

# Create a skeleton directory
mkdir skeleton

# Create a template
cat > skeleton/greetings.txt << EOF
Hello {{ skelly.full_name }}
EOF

# Use it!
cd /tmp
mkdir my_awesome_project
cd my_awesome_project

skelly /path/to/template full_name='John Doe'
```
