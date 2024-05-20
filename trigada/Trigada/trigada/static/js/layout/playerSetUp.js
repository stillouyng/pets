const player = new Plyr('#player');
player.volume = 0.05;
player.speed = 1;
player.loop = true;

const audio = document.getElementById("player");
const audioContext = new AudioContext();
const audioSource = audioContext.createMediaElementSource(audio);
const audioAnalyser = audioContext.createAnalyser(audio);
const numberOfBars = 813;

window.addEventListener("load", () => {
    // turn off preloader
    document.getElementById("preloader").style.display = "none";

    //Get player settings
    const settings = JSON.parse(localStorage.getItem("player_settings"));
    try {
        player.volume = settings['volume'];
        player.speed = settings['speed'];
        player.muted = settings['muted'];
    } catch (err) {}

    // Get song name. This is the biggest piece of dog shit...
    const song_name = player.source.split("/")[player.source.split("/").length - 1].split("-")[1].replace("_", "").split(".")[0];
    // Set currentTime and play
    try {
        const song_data = JSON.parse(localStorage.getItem(song_name));
        player.currentTime = parseInt(song_data['currentTime']);
    } catch (err) {
        player.currentTime = 0;
    }
    player.play();

    const allTime = document.getElementsByClassName("plyr__time plyr__time--current")[0].innerHTML;
    const allTimeInSeconds = (parseInt(allTime.split(":")[0]) * 60 ) + parseInt(allTime.split(":")[1]);

    const timePlayed = setInterval(() => {
        player_settings = {
            "volume": player.volume,
            "speed": player.speed,
            "muted": player.muted
        }

        json_data = {
            "currentTime": player.currentTime
        }
        localStorage.setItem(song_name, JSON.stringify(json_data));
        localStorage.setItem("player_settings", JSON.stringify(player_settings));
    }, 1000);
});

// Connect the source to the analyser
audioSource.connect(audioAnalyser);
audioSource.connect(audioContext.destination);

// Get frequencies
const frequencyData = new Uint8Array(audioAnalyser.frequencyBinCount);
audioAnalyser.getByteFrequencyData(frequencyData);

// Get the visualiser container
const visualiserContainer = document.querySelector(".visualiser-container");

// Create a set of pre-defined bars
//for (let i = 0; i < numberOfBars; i++) {
//    const bar = document.createElement("DIV");
//    bar.setAttribute("id", "bar_" + i);
//    bar.setAttribute("class", "visualiser-container__bar");
//    visualiserContainer.appendChild(bar);
//}

const moonImage = document.getElementById("moon-icon");

// Adjust the bar heights according to the frequency data
function renderBars() {
    audioAnalyser.getByteFrequencyData(frequencyData);

    var sumOfAllValues = 0;

    for (let i = 0; i < numberOfBars; i++) {
        const fd = frequencyData[i];

        sumOfAllValues += fd;

//        const bar = document.querySelector("#bar_" + i);
//        if (!bar) {
//            continue
//        }

//        const barHeight = Math.max(4, fd || 0);
//        bar.style.height = barHeight + "px";

    }

    const blur = Math.floor(sumOfAllValues/numberOfBars);
    const spread = Math.floor(Math.floor(sumOfAllValues/numberOfBars)/5);
    moonImage.style.boxShadow = "inset 0 0 " + (Math.floor(blur/10)) + "px " + (Math.floor(spread/2)) + "px var(--moon-color), 0 0 " + blur + "px " + spread + "px var(--white)";

    window.requestAnimationFrame(renderBars);
}

renderBars();
