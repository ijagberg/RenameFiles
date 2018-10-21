#!/usr/bin/env node

const fse = require('fs-extra');
const readline = require('readline');
var program = require('commander');


program
    .option('-g, --guid', 'Use the GUID naming convention (####-####-####-####, ...). This is the default naming convention used.')
    .option('-i, --incremental', 'Use the incremental naming convention (1, 2, 3, ...)')
    .option('-s, --suppress', 'Suppress prompt for confirmation')
    .parse(process.argv);


async function start() {
    var args = program.args;
    var fileArgs = []
    for (var a in args) {
        var arg = args[a];
        if (await isDirectory(arg)) continue;
        fileArgs.push(arg);
    }
    if (args.length < 1) {
        console.error('No files specified');
        return;
    }

    if (await confirm(fileArgs)) {
        if (!program.guid && !program.incremental) {
            console.log('Using default naming convention (guid)');
            guidRename(fileArgs);
        } else if (!program.guid && program.incremental) {
            console.log('Using incremental naming convention');
            incrementalRename(fileArgs);
        }
    }
    console.log('Bye');
}

async function confirm(fileArgs) {
    if (!program.suppress) {
        var question = 'Randomize the names of the following ' + fileArgs.length + ' files:\n';
        for (var f in fileArgs) {
            var fileArg = fileArgs[f];
            question += '' + fileArg + '\n';
        }
        question += 'Y/N: ';
        var ans = await prompt(question);
        if (ans != 'Y') {
            return false;
        } else {
            return true;
        }
    }
    return true;
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

async function incrementalRename(files) {
    console.error('Incremental rename is not implemented yet!');
    return;
    try {
        for (var i in files) {
            var file = files[i];
            var splitString = file.split('.');
            var fileType = '';
            if (splitString.length > 1) {
                fileType = splitString[splitString.length - 1];
            }
            var newFile = i.toString() + '.' + fileType;
            await fse.move(file, newFile);
        }
    } catch (err) {
        console.error(err);
    }
}

async function guidRename(files) {
    try {
        for (var i in files) {
            var file = files[i];
            var splitString = file.split('.');
            var fileType = '';
            if (splitString.length > 1) {
                fileType = splitString[splitString.length - 1];
            }
            var newFile = generateUUID().toString() + '.' + fileType;
            await fse.move(file, newFile);
        }
    } catch (err) {
        console.error(err);
    }
}

function generateUUID() { // Public Domain/MIT
    var d = new Date().getTime();
    if (typeof performance !== 'undefined' && typeof performance.now === 'function'){
        d += performance.now(); //use high-precision timer if available
    }
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
        var r = (d + Math.random() * 16) % 16 | 0;
        d = Math.floor(d / 16);
        return (c === 'x' ? r : (r & 0x3 | 0x8)).toString(16);
    });
}

start();