#!/usr/bin/env node

const fse = require('fs-extra');
const readline = require('readline');
var program = require('commander');

program
    .option('-r, --restore', 'Undo a previously performed rename operation (requires a .rename-files-restore file in the current directory)')
    .parse(process.argv);

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const start = async function () {
    if (!program.restore) {
        // Rename every file
        await renameAll();
    } else if (program.restore) {
        await restoreAll();
    }
}

function shuffle(array) {
    for (var i = array.length - 1; i > 0; i--) {
        var j = Math.floor(Math.random() * (i + 1));
        var temp = array[i];
        array[i] = array[j];
        array[j] = temp;
    }
}

async function isDirectory(file) {
    var stats = await fse.stat('./' + file);
    return stats.isDirectory();
}

async function renameAll() {
    // Get files from current directory
    var renameFilesRestore = [];
    var files = await fse.readdir('.');
    shuffle(files);
    for (var i in files) {
        var file = files[i];
        if (!await isDirectory(file)) {
            // File is not a directory, so it should be renamed
            var oldFileName = file;
            if (oldFileName == '.rename-files-restore') {
                // Special case for restoration file
                continue;
            }

            var splitString = oldFileName.split('.');
            var fileType = '';
            if (splitString.length > 1) {
                fileType = splitString[splitString.length-1];
            }
            var newFileName = i.toString() + '.' + fileType;

            renameFilesRestore.push({
                newFileName: newFileName,
                oldFileName: oldFileName
            });
        }
    }
    console.log('The following rename-operation will be performed: ');
    console.log(renameFilesRestore);
    rl.question('Y/N: ', async (response) => {
        if (response == 'Y') {
            console.log('Performing rename...');
            await fse.writeJson('./.rename-files-restore', renameFilesRestore);
            for (var i in renameFilesRestore) {
                var entry = renameFilesRestore[i];
                await fse.move(entry["oldFileName"], entry["newFileName"]);
            }
        } else {
            console.log('Ok, cancelling');
        }
        rl.close();
    });
}

start();