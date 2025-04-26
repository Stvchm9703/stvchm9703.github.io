<script>
    import { onMount } from "svelte";
    // import { DateTime } from "luxon";
    import Card from "./card.svelte";
    let tmp = $state([
        {
            id: "R_kgDOIbBing",
            name: "Stvchm9703/stvchm9703",
            description: "profile page - 2022",
            url: "https://github.com/Stvchm9703/stvchm9703",
            languages: {
                Svelte: 49640,
                TypeScript: 7737,
                JavaScript: 7418,
                CSS: 6350,
                HTML: 284,
            },
            tags: [
                "Svelte",
                "TypeScript",
                "JavaScript",
                "CSS",
                "HTML",
                "Profile",
            ],
            updated_at: "2024-06-15T05:24:30Z",
            updated_at_date: 1718429070,
        },
        {
            id: "R_kgDOI_99bg",
            name: "Stvchm9703/uts-32606-database",
            description: "postgress-sql databsae project",
            url: "https://github.com/Stvchm9703/uts-32606-database",
            languages: {
                TypeScript: 1063999,
                JavaScript: 72855,
            },
            tags: ["PostgreSQL", "TypeScript", "Deno", "Prisma", "Backend"],
            license: "MIT License",
            updated_at: "2024-05-29T20:38:01Z",
            updated_at_date: 1717015081,
        },
        {
            id: "R_kgDOLcwTRA",
            name: "Stvchm9703/uts-deep-learning-tutoral-assignment",
            description: "Deep Learning Tutorial and Assignment",
            url: "https://github.com/Stvchm9703/uts-deep-learning-tutoral-assignment",
            languages: {
                "Jupyter Notebook": 20543323,
                Makefile: 1422,
                Python: 357,
            },
            tag: ["Python", "Jupyter Notebook", "Deep Learning", "Pytorch"],
            license: "MIT License",
            updated_at: "2024-05-29T10:12:19Z",
            updated_at_date: 1716977539,
        },
        {
            id: "R_kgDOL1MssQ",
            name: "Stvchm9703/nlp-a3-group-project",
            description: "the nlp a3 group project, the tokenizer masking task",
            url: "https://github.com/Stvchm9703/nlp-a3-group-project",
            languages: {
                Python: 91765,
                "Jupyter Notebook": 40016,
                Makefile: 681,
            },
            license: "MIT License",
            updated_at: "2024-05-16T16:33:29Z",
            updated_at_date: 1715877209,
        },
        {
            id: "R_kgDOLsXo6g",
            name: "Stvchm9703/code-31-game-jam-Shadow-Expel",
            description: "little game jam",
            url: "https://github.com/Stvchm9703/code-31-game-jam-Shadow-Expel",
            languages: {
                ShaderLab: 882686,
                "C#": 454059,
                HLSL: 20629,
                HTML: 4752,
                CSS: 1448,
            },
            tags: ["Unity", "C#", "CSharp", "Game Development", "Game Jam"],
            updated_at: "2024-05-02T07:44:37Z",
            updated_at_date: 1714635877,
        },
        {
            id: "R_kgDOLrxeeg",
            name: "Stvchm9703/xenosphere-rs",
            description: "template engine, for cross-plaform app building",
            url: "https://github.com/Stvchm9703/xenosphere-rs",
            languages: {
                Rust: 2121,
            },
            license: "Apache License 2.0",
            updated_at: "2024-04-09T10:42:00Z",
            updated_at_date: 1712659320,
        },
        {
            id: "R_kgDOLfvy7Q",
            name: "Stvchm9703/image-upscaler-rust",
            description: "Image Upscaler with Rust, and unpacking the SWF file",
            url: "https://github.com/Stvchm9703/image-upscaler-rust",
            languages: {
                Rust: 48412,
            },
            tag: ["Rust", "SWF unpacker", "Image Processing", "Image Upscaler"],
            updated_at: "2024-03-13T11:43:19Z",
            updated_at_date: 1710330199,
        },
        {
            id: "R_kgDOLKAS5A",
            name: "Stvchm9703/stv--helix-config",
            description:
                "helix zmk config, for drawing FF14 and other shortcut",
            url: "https://github.com/Stvchm9703/stv--helix-config",
            languages: {
                C: 16,
            },
            updated_at: "2024-02-01T14:54:18Z",
            updated_at_date: 1706799258,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkyMTQ1MDg3ODE=",
            name: "Stvchm9703/Stvchm9703.github.io__old-nuxt",
            description: "github blogging , for studying and life remind",
            url: "https://github.com/Stvchm9703/Stvchm9703.github.io__old-nuxt",
            languages: {
                HTML: 115410,
                JavaScript: 1511,
            },
            tags: [
                "Nuxtjs",
                "Vue",
                "JavaScript",
                "HTML",
                "CSS",
                "Website",
                "Frontend",
            ],
            updated_at: "2023-12-17T12:34:43Z",
            updated_at_date: 1702816483,
        },
        {
            id: "R_kgDOKFbQwQ",
            name: "Stvchm9703/canvas-note-to-obsidian",
            description:
                "simply convert the module note from canvas to obsidian",
            url: "https://github.com/Stvchm9703/canvas-note-to-obsidian",
            languages: {
                Go: 3299,
            },
            tags: ["Golang", "Canvas", "Obsidian"],
            license: "MIT License",
            updated_at: "2023-09-04T20:54:08Z",
            updated_at_date: 1693860848,
        },
        {
            id: "R_kgDOJAfsRQ",
            name: "Stvchm9703/uts-32130-data-analytics",
            description: "remark node and lab test for data analytics",
            url: "https://github.com/Stvchm9703/uts-32130-data-analytics",
            languages: {
                HTML: 4481684,
                "Jupyter Notebook": 2588825,
                Python: 14615,
            },
            tags: [
                "Python",
                "Jupyter Notebook",
                "Data Analytics",
                "Data Science",
                "Pytorch",
            ],
            license: "MIT License",
            updated_at: "2023-04-08T14:04:15Z",
            updated_at_date: 1680962655,
        },
        {
            id: "R_kgDOI_oHkw",
            name: "Stvchm9703/uts-32555-software-development",
            description:
                "2023 UTS 32555 Assignment project :  Yummy Pizza system ",
            url: "https://github.com/Stvchm9703/uts-32555-software-development",
            languages: {
                Python: 166000,
                JavaScript: 60229,
                TypeScript: 13203,
                Makefile: 960,
                Rust: 582,
                Dockerfile: 575,
                Mako: 510,
                HTML: 500,
            },
            tags: [
                "Python",
                "TypeScript",
                "Docker",
                "Flask",
                "Tauri.app",
                "Solid-js",
            ],
            license: "MIT License",
            updated_at: "2023-03-06T02:08:48Z",
            updated_at_date: 1678068528,
        },
        {
            id: "R_kgDOIuICoQ",
            name: "Stvchm9703/hospital_run",
            description: "2d sprite game",
            url: "https://github.com/Stvchm9703/hospital_run",
            languages: {
                GDScript: 3306,
                GAP: 1470,
            },
            tags: ["GDScript", "GAP", "Game Development", "Godot"],
            license: "MIT License",
            updated_at: "2023-01-04T17:23:01Z",
            updated_at_date: 1672852981,
        },
        {
            id: "R_kgDOIYChhQ",
            name: "Stvchm9703/rewind-card",
            description:
                "(╬▔皿▔)╯[reverse-card] : CSS file to tailwind token, with configurable file. ",
            url: "https://github.com/Stvchm9703/rewind-card",
            languages: {
                Rust: 114371,
            },
            tags: ["Rust", "Tailwind css", "CSS", "Reverse Engineering"],
            license: "MIT License",
            updated_at: "2022-11-29T20:55:44Z",
            updated_at_date: 1669755344,
        },
        {
            id: "R_kgDOH0BlKg",
            name: "Stvchm9703/centanet-v6",
            description: "the trial version of nuxt3 based ",
            url: "https://github.com/Stvchm9703/centanet-v6",
            languages: {
                TypeScript: 350448,
                JavaScript: 37404,
                Vue: 4997,
                CSS: 116,
            },
            tags: ["Nuxtjs", "TypeScript", "JavaScript", "Vue", "CSS"],
            updated_at: "2022-08-13T05:50:13Z",
            updated_at_date: 1660369813,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnk0MDM1NjY0Njc=",
            name: "Stvchm9703/modular",
            description:
                "A test-framework-server , for replacing storybook Webpack base server. ** A Lab server for testing Web Element Component, with Vite and your chemical love **",
            url: "https://github.com/Stvchm9703/modular",
            languages: {},
            license: "Mozilla Public License 2.0",
            updated_at: "2021-09-06T09:43:23Z",
            updated_at_date: 1630921403,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzNjk1NTUyNDc=",
            name: "Stvchm9703/cdt-nlp-chatbot",
            description: "TF Chatbot project, for chat service",
            url: "https://github.com/Stvchm9703/cdt-nlp-chatbot",
            languages: {
                "Jupyter Notebook": 254029,
                Python: 101444,
            },
            tags: ["Python", "Jupyter Notebook", "NLP", "Chatbot", "Pytorch"],
            license: "Creative Commons Zero v1.0 Universal",
            updated_at: "2021-07-05T16:15:10Z",
            updated_at_date: 1625501710,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkyMDk5MjM4MDk=",
            name: "Stvchm9703/unlight-zwei-unity",
            description: "Unity Project for Unlight Zwei",
            url: "https://github.com/Stvchm9703/unlight-zwei-unity",
            languages: {
                "C#": 2637433,
                Starlark: 321393,
                ShaderLab: 37058,
            },
            tags: ["Unity", "C#", "CSharp", "Game Development", "Unlight-Zwei"],
            license: "MIT License",
            updated_at: "2021-06-06T16:46:37Z",
            updated_at_date: 1622997997,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzNTAwMDkxNTI=",
            name: "Stvchm9703/submarine-api",
            description: "the rust based CMS system",
            url: "https://github.com/Stvchm9703/submarine-api",
            languages: {
                Rust: 4251,
            },
            tags: ["Rust", "CMS", "API", "GraphQL"],
            updated_at: "2021-03-28T15:45:43Z",
            updated_at_date: 1616946343,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzNDA4MjMzMDc=",
            name: "Stvchm9703/opencv--image-feature-detect",
            description: "opencv implement for image reg",
            url: "https://github.com/Stvchm9703/opencv--image-feature-detect",
            languages: {
                Python: 36842,
                Makefile: 77,
                Batchfile: 19,
            },
            tags: ["Python", "OpenCV", "Image Reg"],
            updated_at: "2021-03-14T04:06:45Z",
            updated_at_date: 1615694805,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzMDk2OTA5MjY=",
            name: "Stvchm9703/hyper-nuxt",
            description:
                "hybrid SSR Generation for Nuxt SSR, with mutli-language support ",
            url: "https://github.com/Stvchm9703/hyper-nuxt",
            languages: {
                JavaScript: 22275,
                TypeScript: 11758,
                Vue: 639,
            },
            tags: ["Nuxtjs", "SSR", "Vue", "TypeScript", "JavaScript"],
            license: "MIT License",
            updated_at: "2021-02-09T06:15:38Z",
            updated_at_date: 1612851338,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzMzUzMzM3NzE=",
            name: "Stvchm9703/gulp-wp-theme-template",
            description: "wordpress theme with gulp-js bundling ",
            url: "https://github.com/Stvchm9703/gulp-wp-theme-template",
            languages: {},
            tags: ["WordPress", "Gulp", "JavaScript", "CSS", "HTML"],
            license: "GNU General Public License v2.0",
            updated_at: "2021-02-02T15:25:27Z",
            updated_at_date: 1612279527,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzMjQ5NjQzODE=",
            name: "Stvchm9703/TSX-sample",
            description: "trial of typescript react",
            url: "https://github.com/Stvchm9703/TSX-sample",
            languages: {
                JavaScript: 4866,
                HTML: 1406,
                TypeScript: 1170,
                SCSS: 189,
            },
            tags: [
                "TypeScript",
                "JavaScript",
                "CSS",
                "HTML",
                "React",
                "Frontend",
            ],
            license: "MIT License",
            updated_at: "2020-12-28T09:45:01Z",
            updated_at_date: 1609148701,
        },
        {
            id: "MDEwOlJlcG9zaXRvcnkzMTUzNDU5NTM=",
            name: "Stvchm9703/WSConsoleServer",
            description: "Rust Base remote log service, via the websocket ",
            url: "https://github.com/Stvchm9703/WSConsoleServer",
            languages: {
                Rust: 8347,
            },
            tags: ["Rust", "WebSocket", "Remote Log"],
            license: "GNU Lesser General Public License v2.1",
            updated_at: "2020-11-23T14:50:20Z",
            updated_at_date: 1606143020,
        },
    ]);
    // onMount(async () => {
    //   let importData = (await import(`$assets/content/full-repo-data.json`)).default;
    //   // console.log(importData);
    //   importData = importData
    //   .filter((elm)=>elm.description !== null)
    //   .map((elm) => ({
    //     ...elm,
    //     updated_at_date: DateTime.fromISO(elm.updated_at).toUnixInteger(),
    //   }));
    //   importData = importData.sort((a, b) => b.updated_at_date - a.updated_at_date);
    //   tmp = importData.slice(0,24);
    //   console.log(tmp)
    // });
</script>

<div
    class="relative grid gap-2 w-full h-auto flex-1 2xl:grid-cols-6 md:grid-cols-3 2xl:grid-rows-5"
>
    {#each tmp as item}
        <Card {item} />
    {/each}
</div>
