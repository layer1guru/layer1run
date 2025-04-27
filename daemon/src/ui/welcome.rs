/**
 * Welcome Banner (Graffiti)
 *
 * This is the default artwork printed as a welcome banner
 * when the CLI is executed.
 */
pub fn banner() {
    println!(r"

    _______             .________________________
    \      \   ____   __| _/\_   _____/\______   \__ __  ___________
    /   |   \ /  _ \ / __ |  |    __)_  |       _/  |  \/    \_  __ \
   /    |    (  <_> ) /_/ |  |        \ |    |   \  |  /   |  \  | \/
   \____|__  /\____/\____ | /_______  / |____|_  /____/|___|  /__|
           \/            \/         \/         \/           \/       ");

    println!("{:>72}\n", noderunr::get_version());
    println!("{:>72}", "brought to you by L1 GÜRŲ");
    println!("{:>72}", "https://layer1.guru");
}

/**
 * Welcome Banner (ANSI Shadow)
 *
 * TBD
 */
pub fn banner_alt() {
    println!(r"

    ███╗   ██╗ ██████╗ ██████╗ ███████╗██████╗ ██╗   ██╗███╗   ██╗██████╗
    ████╗  ██║██╔═══██╗██╔══██╗╚══════╝██╔══██╗██║   ██║████╗  ██║██╔══██╗
    ██╔██╗ ██║██║   ██║██║  ██║ █████╗ ██████╔╝██║   ██║██╔██╗ ██║██████╔╝
    ██║╚██╗██║██║   ██║██║  ██║ ╚════╝ ██╔══██╗██║   ██║██║╚██╗██║██╔══██╗
    ██║ ╚████║╚██████╔╝██████╔╝███████╗██║  ██║╚██████╔╝██║ ╚████║██║  ██║
    ╚═╝  ╚═══╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝");

    println!("{:>72}\n", noderunr::get_version());
    println!("{:>72}", "brought to you by L1 GÜRŲ");
    println!("{:>72}", "https://layer1.guru");
}

/**
 * Welcome Banner (Crawford)
 *
 * TBD
 */
pub fn banner_alt_2() {
    println!(r"

    ____    ___   ___      ___  ____   __ __  ____   ____
    |    \  /   \ |   \    /  _]|    \ |  T  T|    \ |    \
    |  _  YY     Y|    \  /  [_ |  D  )|  |  ||  _  Y|  D  )
    |  |  ||  O  ||  D  YY    _]|    / |  |  ||  |  ||    /
    |  |  ||     ||     ||   [_ |    \ |  :  ||  |  ||    \
    |  |  |l     !|     ||     T|  .  Yl     ||  |  ||  .  Y
    l__j__j \___/ l_____jl_____jl__j\_j \__,_jl__j__jl__j\_j");

    println!("{:>72}\n", noderunr::get_version());
    println!("{:>72}", "brought to you by L1 GÜRŲ");
    println!("{:>72}", "https://layer1.guru");
}
