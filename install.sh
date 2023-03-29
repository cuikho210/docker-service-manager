#!/usr/bin/sh
echo "Begin."

echo "Start build"
cargo build --release
install_path="${HOME}/.local/bin"

if [ "$1" ]
then
    install_path="$1"
fi

if [ ! "$(echo $PATH | grep $install_path)" ]
then
    if [[ -f "${HOME}/.bashrc" && ! "$(grep $install_path ~/.bashrc)" ]]
    then
        echo "export PATH=\"\${PATH}:${install_path}\"" >> "${HOME}/.bashrc"
        echo "Added environment variable to ~/.bashrc"
    else
        echo "Path for ${install_path} already exists in ~/.bashrc"
    fi

    if [[ -f "${HOME}/.zshrc" && ! "$(grep $install_path ~/.zshrc)" ]]
    then
        echo "export PATH=\"\${PATH}:${install_path}\"" >> "${HOME}/.zshrc"
        echo "Added environment variable to ~/.zshrc"
    else
        echo "Path for ${install_path} already exists in ~/.zshrc"
    fi
else
    echo "${install_path} already exists in \$PATH"
fi

if [ ! -f "${install_path}/dsm" ]
then
    mkdir -p "$install_path"
    mv "./target/release/docker-service-manager" "${install_path}/dsm"
    echo "Moved ./target/release/docker-service-manager to ${install_path}/dsm"
else
	rm "${install_path}/dsm"
	mv "./target/release/docker-service-manager" "${install_path}/dsm"
    echo "Overwritten dsm binary in ${install_path}/dsm"
fi

echo "End."
