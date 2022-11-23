const ffi = require("ffi-napi");

const lib = ffi.Library("../rust-part/target/release/librust_part", {
	parseCsv: ["string", []],
});

let result = lib.parseCsv();

let toJson = JSON.parse(result);

console.log(toJson);