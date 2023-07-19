const res = await fetch("https://emoji-auth.shuttleapp.rs/", {
  headers: {
    Authorization: `Emoji ${encodeURIComponent("🐱🐶🐧🐤")}`,
  },
});
const text = await res.text();
console.log(text);
