#!/usr/bin/env zx
import 'zx/globals';

const fixturesDir = 'program/tests/fixtures';
const tempDir = '.cloned-programs';

async function cloneProgram(programName) {
    const repoUrl = `https://github.com/paladin-bladesmith/${programName}-program.git`;
    const targetDir = `${tempDir}/${programName}`;
    const programBinaryName = `paladin_${programName}_program.so`;

    echo(chalk.green(`Cloning and building ${programName} program...`));

    // Check if the target directory exists.
    if (fs.existsSync(targetDir)) {
        // If the program hasn already been cloned, pull it.
        echo(chalk.blue(`Directory ${targetDir} already exists. Pulling latest changes...`));
        await $`git -C ${targetDir} pull`.quiet();
    } else {
        // If the program hasn't been cloned yet, clone it.
        echo(chalk.blue(`Directory ${targetDir} does not exist. Cloning...`));
        await $`git clone ${repoUrl} ${targetDir}`.quiet();
    }

    // Build the program.
    await $`cargo build-sbf --features bpf-entrypoint --manifest-path ${targetDir}/program/Cargo.toml`;
    
    // Move the compiled program into `fixtures`.
    await $`cp -f ${targetDir}/target/deploy/${programBinaryName} ${fixturesDir}/`.quiet();
}

await $`mkdir -p ${fixturesDir}`.quiet();
await $`mkdir -p ${tempDir}`.quiet();

await cloneProgram('rewards');
await cloneProgram('stake');