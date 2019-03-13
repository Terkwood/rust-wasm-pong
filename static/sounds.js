function createAudio (src) {
  try {
    var a = new Audio (src);
    a.volume = 0.1; // lets be real quiet please
    return a;
  } catch (e) {
    return null;
  }
}
var pongSounds = {
  files: {
    ping: createAudio ('/ping.wav'),
    pong: createAudio ('/pong.wav'),
    goal: createAudio ('/goal.wav'),
  },
  play: function (name) {
    if (this.files[name]) this.files[name].play ();
  },
  ping: function () {
    this.play ('ping');
  },
  pong: function () {
    this.play ('pong');
  },
  goal: function () {
    this.play ('goal');
  },
};
