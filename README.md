<a name="readme-top"></a>
<!--
*** Thanks for checking out the rust-api. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">🦀 Rust API 🦀</h3>
<p align="center">
A Rust API template, I built to learn Rust and the Rust ecosystem.   
</p>
</div>


<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#startup">Running the App</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <!-- <li><a href="#acknowledgments">Acknowledgments</a></li> -->
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Simple Rust API template, I built to learn Rust and the Rust ecosystem.

why?
- to learn Rust
- Rust is fast
- no need for a virtual environment
- did I mention it's fast?


<p align="right">(<a href="#readme-top">back to top</a>)</p>



### **Built With**

[![Rust-shield]][Rust-url]
* [![Diesel-shield]][Diesel-url]
* [![Actix-shield]][Actix-url]
* [![cargo-watch-shield]][cargo-watch-url]

[![Postgres-shield]][Postgres-url]

[![Docker-shield]][Docker-url]

[![Docker-Compose-shield]][Docker-Compose-url]
<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## **Getting Started**

### Prerequisites

* Project Dependencies
  *  [Rust](https://www.rust-lang.org/), main programming language for the application
  *  [PostgreSQL](https://www.postgresql.org/), sql database
  *  [Diesel](http://diesel.rs/), ORM for Rust
  *  [Actix](https://actix.rs/), web framework for Rust
  * [cargo-watch](https://github.com/watchexec/cargo-watch), watches for changes in code during development

* Development Dependencies
    *  [Docker](https://www.docker.com/), containerization platform
    *  [Docker Compose](https://docs.docker.com/compose/), tool for defining and running multi-container Docker applications

### Startup

* clone the repo
    ```sh
    git clone https://github.com/aguerrero232/rust-api.git rust-api
    ```

* change into the projects directory

    ```sh
    cd rust-api
    ```

* build the projects app container

    ```sh
    docker-compose build
    ```

* run the projects containers

    ```sh
    docker-compose up
    ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

See the [open issues](https://github.com/aguerrero232/rust-api) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Guerrero Ariel - arielguerrero1012@gmail.com


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/aguerrero232/rust-api.svg?style=for-the-badge

[contributors-url]: https://github.com/aguerrero232/rust-api/graphs/contributors

[forks-shield]: https://img.shields.io/github/forks/aguerrero232/rust-api.svg?style=for-the-badge

[forks-url]: https://github.com/aguerrero232/rust-api/network/members

[stars-shield]: https://img.shields.io/github/stars/aguerrero232/rust-api.svg?style=for-the-badge
[stars-url]: https://github.com/aguerrero232/rust-api/stargazers

[issues-shield]: https://img.shields.io/github/issues/aguerrero232/rust-api.svg?style=for-the-badge

[issues-url]: https://github.com/aguerrero232/rust-api/issues

[license-shield]: https://img.shields.io/github/license/aguerrero232/rust-api.svg?style=for-the-badge
[license-url]: https://github.com/aguerrero232/rust-api/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/arielguerrero1012/
[product-screenshot]: images/screenshot.png
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[Vue.js]: https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vuedotjs&logoColor=4FC08D
[Vue-url]: https://vuejs.org/
[Angular.io]: https://img.shields.io/badge/Angular-DD0031?style=for-the-badge&logo=angular&logoColor=white
[Angular-url]: https://angular.io/
[Svelte.dev]: https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00
[Svelte-url]: https://svelte.dev/
[Laravel.com]: https://img.shields.io/badge/Laravel-FF2D20?style=for-the-badge&logo=laravel&logoColor=white
[Laravel-url]: https://laravel.com
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com 


[Rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Postgres-shield]: https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white
[Postgres-url]: https://www.postgresql.org/
[Docker-shield]: https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white
[Docker-url]: https://www.docker.com/
[Docker-compose-shield]: https://img.shields.io/badge/Docker%20Compose-007ACC?style=for-the-badge&logo=docker&logoColor=white
[Docker-compose-url]: https://docs.docker.com/compose/
[Diesel-shield]: https://img.shields.io/badge/Diesel-000000?style=for-the-badge&logo=rust&logoColor=white
[Diesel-url]: http://diesel.rs/
[Actix-shield]: https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=rust&logoColor=white
[Actix-url]: https://actix.rs/
[Cargo-watch-shield]: https://img.shields.io/badge/Cargo%20Watch-000000?style=for-the-badge&logo=rust&logoColor=white
[Cargo-watch-url]: https://github.com/watchexec/cargo-watch