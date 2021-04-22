import * as rn from "random-name";
import * as $ from 'jquery';

// const result = rn.random_name(['a', 'b', 'c']);
// console.log(result);

var running = false;

$("body").on("click", (e) => {
  if (running) return;
  running = true;
  $("body").removeClass("end");
  $("#pow").removeClass("hidden");
  $("#winner").removeClass("in");
  ($("#sound")[0] as HTMLAudioElement).currentTime = 0;
  ($("#bomb")[0] as HTMLAudioElement).currentTime = 0.6;
  ($("#sound")[0] as HTMLAudioElement).play();
  ($("#bomb")[0] as HTMLAudioElement).pause();
  $("body").addClass("start");
  setTimeout(end, 9000);
});

const end = () => {
  $("body").addClass("end");
  $("body").removeClass("start");
  ($("#sound")[0] as HTMLAudioElement).pause();
  ($("#bomb")[0] as HTMLAudioElement).play();
  setTimeout(removeBomb, 1000);
};

const removeBomb = () => {
  $("#pow").addClass("hidden");
  //const idx = Math.random() * uniques.length;
  console.log('%cCalling into Rust 🦀', 'color:blue; font-size:35px');
  const winner = rn.random_name(uniques);
  console.log('%cGot winner from Rust 🦀', 'color:blue; font-size:35px');
  const idx = uniques.indexOf(winner);
  $("#winner").text(winner);//uniques[idx]);
  $("#winner").addClass("in");
  running = false;
  uniques.splice(idx, 1);
};

const meetup = ["Abhik Jain", "Adam R", "Adrien Chardon", "Alexander Entinger", "Alexander Kabui", "Andreas Frühwirt", "Bram Geron", "Canberk", "Ciara", "Cihat Uysal", "Cole Heslinga", "Cosmin Călinescu", "Daniel Blanco", "David García Morillo", "Ewald Horn", "extrawurst", "Fede", "Florian Schaupp", "Fred Morcos", "Gilbert Röhrbein", "Glyn Matthews", "GuillerLT", "Gwan-gyeong Mun", "Harald Reingruber", "Herbert Wolverson", "Hussein", "Jan-Erik", "Javier Viola", "Jeffrey", "Karl Grasegger", "Karsten Katze", "Kaviraj Kanagaraj", "kofi nertey hervie", "Kostas Papakonstantinou", "Krešimir Tonković", "marius armenteras", "Mark Dodgson", "Markus M. Egger", "Markus Stolze", "Martin Grotz", "Martin Pollhammer", "Matthias Geier", "Meet Vora", "Michael", "Michael Druk", "Mingwei Zhang", "Neeraj Yadav", "Nguyen Duc", "Norbert Kehrer", "Nuno Martins", "Oliver", "Otmar Pilgerstorfer", "oylenshpeegul", "Peter Donner", "Peter Kehl",  "Ramnivas Laddad", "Ricardo Almeida", "Roberto Huertas", "Samuel", "Scott Fertig", "Sergio Britos", "Shady Khalifa", "Sirish Kumar",  "Tomasz Waszczyk", "Tuta Muniz", "vibi", "Wojciech Żurek", "Wolfgang Ziegler", "Zeeshan Ali"];

const members = meetup.concat([]);

const uniques = Array.from(new Set(members));
