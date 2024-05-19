const player = new Plyr('#player');
player.volume = 0.05;
player.speed = 1;
player.loop = true;

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
