export default function buildJsonTree(paths) {
    const root = {
        name: "root",
        type: "folder",
        description: "Source files for project",
        children: []
    };

    paths.forEach(path => {
        const parts = path.split('/').slice(5); // Adjust the slice to match the depth of your initial folder
        let currentLevel = root.children;

        parts.forEach((part, index) => {
            let existingPath = currentLevel.find(p => p.name === part);

            if (!existingPath) {
                if (index === parts.length - 1) { // It's a file
                    existingPath = {
                        name: part,
                        type: 'file',
                        description: '',
                        children: []
                    };
                } else { // It's a folder
                    existingPath = {
                        name: part,
                        type: 'folder',
                        description: '',
                        children: []
                    };
                }
                currentLevel.push(existingPath);
            }
            currentLevel = existingPath.children;
        });
    });

    return root;
}

