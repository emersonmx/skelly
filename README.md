# skelly

A simple scaffold tool written in Rust.

## Usage

```
Usage: skelly [OPTIONS] <SKELETON_PATH> [INPUTS]...

# Output a skeleton to files
skelly -s ~/path/to/skeleton input1=one input2=two inputN=N

# Output a skeleton to stdout
skelly -s ~/path/to/skeleton input1=one input2=two inputN=N | cat

# Output a template from stdin to stdout
echo "{{ name }} <{{ email }}>" | skelly name=john email=john@example.com 
```

## Template creation and usage

```sh
# Create a template directory
mkdir /tmp/my_template
cd /tmp/my_template

# Create a skelly.toml
cat > skelly.toml << EOF
[[inputs]]
name = "full_name"
EOF

# Create a skeleton directory
mkdir skeleton

# Create a template file
cat > skeleton/greetings.txt << EOF
Hello {{ skelly.full_name }}
EOF

# Use it!
mkdir /tmp/my_awesome_project
cd /tmp/my_awesome_project

skelly -s /tmp/my_template full_name='John Doe'
```
