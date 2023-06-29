const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");

var packageDefinition = protoLoader.loadSync("../proto/service.proto", {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true
});

const protoDescriptor = grpc.loadPackageDefinition(packageDefinition);

const service = protoDescriptor.service;

const client = new service.Apis("localhost:1370", grpc.credentials.createInsecure());
client.test({ value: "JS Test User" }, (err, response) => {
    if (err) {
        console.error("Error: ", err);
    } else {
        console.log(response);
    }
});
