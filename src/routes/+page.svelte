<script>
  // export const p`rerender = true;

  import Navbar from "$lib/navbar/index-page.svelte";
  import Banner from "$lib/banner.svelte";
  import BannerGridCell from "$lib/banner-grid-cell.svelte";

  import Footer from "$lib/footer.svelte";
  // import SampleImage from "$assets/img/placeholder-hero.jpg?format=webp&w=750;900;1200;2400&q=85&as=metadata";
  import IndexImage_L0 from "$assets/series/index-page/cover_l0.png?format=webp&w=1200;1800;2400&q=85&as=metadata";
  //   import IndexImage_L1 from "$assets/series/index-page/cover_l1.png?format=webp&w=1800;2400;3000&q=85&as=metadata";
  import BaseImage_1 from "$assets/img/profile.jpg?format=webp&w=700;1200&q=85&as=metadata";
  import BaseImage_2 from "$assets/img/pepper_dayly.jpg?format=webp&w=700;1200&q=85&as=metadata";
  import BaseImage_3 from "$assets/img/centaline_groupmate.jpg?format=webp&w=700;1200&q=85&as=metadata";
  import BaseImage_4 from "$assets/img/coffee.jpg?format=webp&w=700;1200&q=85&as=metadata";
  import BaseImage_5 from "$assets/img/2024_uts.jpg?format=webp&w=700;1200&q=85&as=metadata";

  import KeyboardImage from "$assets/img/keyboard.jpg?format=webp&q=85";
  import GameAnimation from "$assets/animation/alicetaria-walking.webm?url&format=webm";
  import ScrollFrame from "$lib/scroll-frame.svelte";
  // import SkillTree from "$lib/skill-tree/elk/main.svelte";
  import SkillV2 from "$lib/skillset/main.svelte"
  import ProjectSet from "$lib/project-set/card-list.svelte";
  import ImageSet from "$lib/Image.svelte";

  // import { onMount } from "svelte";

  let currentSection = "index";
</script>

<svelte:head>
  <title>Steven, the web dev</title>
  <!-- <meta /> -->
</svelte:head>

<Navbar activeId={currentSection} />

<Banner
  id="index"
  isFullScreen
  containerClass="h-full grid items-center content-center"
>
  {#snippet background()}
    <div class="inset-0 absolute">
      <ImageSet
        class="inset-0 absolute z-1"
        srcList={IndexImage_L0}
        forceLoad
        alt="index banner image; layer 0"
      />
      <!-- <ImageSet
              class="inset-0 absolute z-2"
              srcList={IndexImage_L1}
              forceLoad
              alt="index banner image; layer 1"
          /> -->
    </div>
  {/snippet}
  <BannerGridCell align="left">
    <h1 class="text-light-50 text-5xl leading-loose">Steven - Cheng Ho Man</h1>
    <p class="text-light-200 leading-loose">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
</Banner>

<Banner
  id="introduction"
  class="w-full my-6"
  containerClass="flex flex-col xl:grid  xl:grid-cols-2 2xl:grid-cols-3 justify-items-stretch  grid-rows-auto grid-content-start gap-2 lg:gap-y-8  pt-32"
>
  {#snippet background()}
    <ScrollFrame class="bg-dark-100 inset-0 bg-dark-100 fixed -z-1 " />
  {/snippet}

  <h1
    class="grid-row-start-1 2xl:( grid-col-start-2 grid-col-end-2) xl:(grid-col-start-1 grid-col-end-3) text-center text-light-50 text-5xl mt-6 mb-3"
  >
    Who I am ...
  </h1>
  <BannerGridCell align="left" row={2}>
    <h2 class="text-light-50 text-3xl">A Man who run with code</h2>
    <p class="text-light-200">almost, but not quite. Highly motivated and detail-oriented software developer with 3+ years of experience in designing, developing, and deploying cutting-edge software solutions. Proficient in Javascript, C#, Golang with expertise in Web Application. Proven track record of delivering high-quality projects on time, with a strong passion for innovation and continuous learning.</p>
  </BannerGridCell>
  <h1
    class="grid-row-start-4 2xl:( grid-col-start-2 grid-col-end-2) xl:(grid-col-start-1 grid-col-end-3) text-center text-light-50 text-5xl mt-6 mb-3"
  >
    In Career
  </h1>
  <BannerGridCell align="right" row={5} class="flex mt-0 mb-a self-start">
    <div class="aspect-video">
      <span class="text-light-700"> September 2015 - July 2017 </span>
      <h2 class="text-light-50 text-3xl">Student</h2>
      <ul class="text-light-200">
        <li>
          <p>Associate of Science in Information Systems Development</p>
        </li>
        <li>
          <p>Community College Of City University</p>
        </li>
        <li>
          <p>Major: Computer Programming</p>
        </li>
      </ul>
    </div>
  </BannerGridCell>
  <BannerGridCell align="left" row={5} class="!p-0">
    <ImageSet
      class="aspect-video w-full block"
      srcList={BaseImage_1}
      alt="myself image"
    />
  </BannerGridCell>

  <BannerGridCell align="right" row={6} class="flex mt-0 mb-a self-start">
    <div class="aspect-video">
      <span class="text-light-700"> November 2017 - May 2019 </span>
      <h2 class="text-light-50 text-3xl">Web Programmer</h2>
      <ul class="text-light-200">
        <li class="my-2">
          <p class="text-lg text-500">Teksbotics (Hong Kong) Limited</p>
        </li>
        <li>
          <p>
            Teksbotics (Hong Kong) Limited is an investment, consulting, and
            technology commercialization company based in Hong Kong. It
            specializes in robotics and artificial intelligence technologies.
            The company’s vision is to become one of the leading robotics and
            artificial intelligence firms in the Asia Pacific region. Teksbotics
            (Hong Kong) Limited aims to create value for its customers,
            suppliers, partners, employees, and shareholders while serving as a
            crucial link between robotics and AI companies and the Chinese
            financial, industrial, and consumer markets.
          </p>
        </li>
      </ul>
    </div>
  </BannerGridCell>
  <BannerGridCell align="left" row={6} class="!p-0">
    <ImageSet
      class="aspect-video w-full block children:grayscale-100"
      srcList={BaseImage_2}
      alt="myself image"
    />
  </BannerGridCell>

  <BannerGridCell align="right" row={7} class="flex mt-0 mb-a self-start">
    <div class="aspect-video">
      <span class="text-light-700"> September 2017 - July 2020 </span>
      <h2 class="text-light-50 text-3xl">Student</h2>
      <ul class="text-light-200">
        <li>
          <p>Bachelor of Science (Hons) Computing Science</p>
        </li>
        <li>
          <p>School Of Continuing and Professional Education</p>
        </li>
      </ul>
    </div>
  </BannerGridCell>

  <BannerGridCell align="right" row={8} class="flex mt-0 mb-a self-start">
    <div class="aspect-video">
      <span class="text-light-700"> January 2021 - November 2022 </span>
      <h2 class="text-light-50 text-3xl">Frontend Developer</h2>
      <ul class="text-light-200">
        <li class="my-1">
          <p class="text-lg text-500">Centaline Data Technology Limited</p>
        </li>
        <li>
          <p>
            Centaline Data Technology Limited is a subsidiary of The Centaline
            Group, a company based in Hong Kong. It is a service company
            specializing in Information Technology. Established in 2017,
            Centaline Data Technology Limited operates in the major market of
            Hong Kong with a staff size ranging from 51 to 100 employees. The
            company offers career opportunities and competitive remuneration
            packages for individuals looking to join their team.
          </p>
        </li>
      </ul>
    </div>
  </BannerGridCell>
  <BannerGridCell align="left" row={8} class="!p-0">
    <ImageSet
      class="aspect-video w-full block children:grayscale-100"
      srcList={BaseImage_3}
      alt="myself image"
    />
  </BannerGridCell>

  <BannerGridCell align="right" row={9} class="flex mt-0 mb-a self-start">
    <div class="aspect-video">
      <span class="text-light-700"> January 2023 - December 2024 </span>
      <h2 class="text-light-50 text-3xl">Fulltime Student</h2>
      <p class="text-light-200">
        <span class="text-lg text-500"
          >Master of Information System, University of Technology Sydney</span
        >
        <span>
          Mainly studying in Artifactial Inteligel, Machine Learning, and Deep
          Learning.
        </span>
      </p>
    </div>
  </BannerGridCell>
  <BannerGridCell align="left" row={9} class="!p-0">
    <ImageSet
      class="aspect-video w-full block children:grayscale-100"
      srcList={BaseImage_5}
      alt="myself image"
    />
  </BannerGridCell>

<!--  -->

  <h1 class="grid-row-start-11 grid-col-start-2 grid-col-end-2 text-center text-light-50 text-5xl mt-6 mb-3" > Hobby  </h1>
  <BannerGridCell align="left" row={12}>
    <h2 class="text-light-50 text-3xl">Drawing</h2>
    <div class="text-light-50 text-lg">
      just a little bit, not much, but I like it
      <div class="grid w-24 h-24"></div>
    </div>
  </BannerGridCell>
  <BannerGridCell align="right" row={13}>
    <h2 class="text-light-50 text-3xl">Program making</h2>
    <div class="text-light-50 text-lg">
      make some small util tools for myself, to make my life easier; but not a
      big fans of code leeting
      <div class="grid w-24 h-24"></div>
    </div>
  </BannerGridCell>
  <BannerGridCell align="left" row={14}>
    <h2 class="text-light-50 text-3xl">Game making</h2>
    <div class="text-light-50 text-lg">
      make some small games, but not a big fans of game playing
    </div>
  </BannerGridCell>
  <BannerGridCell align="right" row={14} class="!p-0 !backdrop-filter-none">
    <video
      class="aspect-square w-full block max-h-4xl"
      src={GameAnimation}
      preload="none"
      alt="animation"
      loop="true"
      autoplay="true"
    >
    <track kind="captions"/>
  </video>
  </BannerGridCell>
  <BannerGridCell align="right" row={16}>
    <h2 class="text-light-50 text-3xl">Keyboard</h2>
    <p class="text-light-50 text-lg">
      nice to type on, nice to look at. speed up my typing speed; also, make for
      cool stuff
    </p>
    <img
      class="mt-5 w-full min-h-24 object-contain aspect-ratio-video"
      src={KeyboardImage}
      alt=""
    />
  </BannerGridCell>

  <BannerGridCell align="left" row={17}>
    <h2 class="text-light-50 text-3xl">Coffee(?)</h2>
    <div class="text-light-50 text-lg">coffee energy, enpower me everyday</div>
    <!-- <img class="mt-5 w-full min-h-24 object-contain" src={BaseImage_2} alt="" /> -->
    <ImageSet
      class="mt-5 w-full min-h-24 object-contain aspect-ratio-video rounded"
      srcList={BaseImage_4}
      alt="coffee image"
    />
  </BannerGridCell>
</Banner>

<Banner
  id="project"
  isFullScreen
  containerClass="!max-w-99rem h-full"
  class="mt-6 w-full"
>
  {#snippet background()}
    <div class=" backdrop-blur-lg inset-0 absolute"></div>
  {/snippet}
  <div
    class="flex flex-col self-center max-w-full items-start w-full h-full max-h-screen"
  >
    <div class="px-6 py-4 rounded-lg bg-dark-700/45 mb-6">
      <h1 class="text-3xl text-light-900">Project</h1>
    </div>
    <ProjectSet />
  </div>
</Banner>

<!--  -->
<Banner id="skill" isFullScreen containerClass="!max-w-99rem">
  {#snippet background()}
    <div class=" backdrop-blur-lg inset-0 absolute"></div>
  {/snippet}
  <div class="flex flex-col self-center max-w-full items-start">
    <div class="px-6 py-4 rounded-lg bg-dark-700/45 mb-3">
      <h1 class="text-3xl text-light-900">Skill Set</h1>
      <p class="text-light-800">
        Those I known and those I am learning, all skill and tech are mapped on
        the canvas
      </p>
    </div>
    <SkillV2 />
  </div>
</Banner>

<Banner id="contact" containerClass="z-1" class="z-1">
  {#snippet background()}
    <div class=" absolute inset-0 bg-blue-900 flex flex-col"></div>
  {/snippet}

  <div class="flex flex-col 2xl:min-h-150">
    <h1 class="text-center 2xl:text-3xl text-xl text-light-50 font-light">
      Contact Me
    </h1>
    <div class="mt-8 max-w-xl flex flex-col w-full self-center">
      <table class="table-auto">
        <tbody>
          <tr class=" border-b-1 border-white">
            <td class="py-3 2xl:text-2xl text-lg text-light-50 font-light"
              >Email:</td
            >
            <td class="py-3 flex justify-end">
              <a
                class="text-right 2xl:text-2xl text-lg text-light-50 font-light"
                href="mailto:st.ch.inwork@gmail.com"
              >
                st.ch.inwork@gmail.com
              </a>
            </td>
          </tr>

          <tr class=" border-b-1 border-white">
            <td class="py-3 flex flex-col">
              <span class="2xl:text-2xl text-lg text-light-50 font-light"
                >Tel:</span
              >
            </td>
            <td class="py-3">
              <div class="flex flex-col">
                <a
                  class="text-right 2xl:text-2xl text-lg text-light-50 font-light"
                  href="tel:+85297915541"
                >
                  +852-97915541
                </a>
                <a
                  class="text-right 2xl:text-2xl text-lg text-light-50 font-light"
                  href="tel:+86435397917"
                >
                  +86-435397917
                </a>
              </div>
            </td>
          </tr>

          <tr class=" border-b-1 border-white">
            <td class="py-3 flex flex-col">
              <span class="2xl:text-2xl text-lg text-light-50 font-light"
                >LinkedIn:</span
              >
            </td>
            <td class="py-3">
              <div class="flex flex-col">
                <a
                  class="text-right 2xl:text-2xl text-lg text-light-50 font-light"
                  href="https://www.linkedin.com/in/chengsteven430431182/"
                  target="_blank"
                >
                  Steven Cheng
                </a>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</Banner>

<Footer />
