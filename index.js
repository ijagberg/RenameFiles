#!/usr/bin/env node

const fse = require('fs-extra');
const readline = require('readline');
var program = require('commander');

program
    .option('-r, --restore', 'Undo a previously performed rename operation (requires a .rename-files-restore file in the current directory)')
    .parse(process.argv);


async function start() {
    var files = await getFilesInCurrentDirectory();
    if (!program.restore) {
        await renameAll(files);
    } else if (program.restore) {
        await restoreAll(files);
    }
    console.log('Bye');
}

function shuffle(array) {
    for (var i = array.length - 1; i > 0; i--) {
        var j = Math.floor(Math.random() * (i + 1));
        var temp = array[i];
        array[i] = array[j];
        array[j] = temp;
    }
}

async function getFilesInCurrentDirectory() {
    try {
        var filesAndDirs = await fse.readdir('.');
        var files = [];
        for (var i in filesAndDirs) {
            var fileDir = filesAndDirs[i];
            if (!await isDirectory(fileDir)) {
                files.push(fileDir);
            }
        }
        return files;
    } catch (err) {
        console.error(err);
    }
}

async function isDirectory(file) {
    var stats = await fse.stat('./' + file);
    return stats.isDirectory();
}

async function prompt(query) {
    var rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });
    var f = async function () {
        return new Promise(function (resolve, reject) {
            rl.question(query, resolve);
        });
    };
    var answer = await f();
    rl.close();
    return answer;
}

async function renameAll(files) {
    try {
        // Get files from current directory
        for (var i in files) {
            var file = files[i];
            var oldFileName = file;
            if (oldFileName == '.rename-files-restore') {
                // Special case for restoration file
                continue;
            }

            var splitString = oldFileName.split('.');
            var fileType = '';
            if (splitString.length > 1) {
                fileType = splitString[splitString.length - 1];
            }
            var newFileName = i.toString() + '.' + fileType;

            renameFilesRestore.push({
                newFileName: newFileName,
                oldFileName: oldFileName
            });
        }
        console.log('The following rename operation will be performed: ');
        console.log(renameFilesRestore);
        var answer = await prompt('Y/N: ');
        if (answer == 'Y') {
            console.log('Performing rename...');
            await fse.writeJson('./.rename-files-restore', renameFilesRestore);
            for (var i in renameFilesRestore) {
                var entry = renameFilesRestore[i];
                await fse.move(entry["oldFileName"], entry["newFileName"]);
            }
        }
    } catch (err) {
        console.error(err);
    }
}


async function restoreAll() {
    try {
        var renameFilesRestore = await fse.readJSON('./.rename-files-restore');
        console.log('The following restore operation will be performed: ');
        console.log(renameFilesRestore);
        var answer = await prompt('Y/N: ');
        if (answer == 'Y') {
            console.log('Performing restore...');
            for (var i in renameFilesRestore) {
                var entry = renameFilesRestore[i];
                await fse.move(entry["newFileName"], entry["oldFileName"]);
            }
            console.log('Restoration complete, removing .rename-files-restore...');
            await fse.remove('./.rename-files.restore');
        }
    } catch (err) {
        console.error(err);
    }
}

start();