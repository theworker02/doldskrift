import { encode, decode, Session } from "../../packages/doldskrift-js/dist/index.js";

const text = process.argv[2] ?? "hello agent";
const opaque = encode(text);
console.log("encoded:", opaque);
console.log("decoded:", decode(opaque));

const session = Session.builder().seed("demo").build();
const sess = session.encode(text);
console.log("session:", sess);
console.log("session decoded:", session.decode(sess));
