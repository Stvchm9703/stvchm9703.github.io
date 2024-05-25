<script>
  import Navbar from "$lib/navbar/index-page.svelte";
  import Banner from "$lib/banner.svelte";
  import BannerGridCell from "$lib/banner-grid-cell.svelte";

  import Footer from "$lib/footer.svelte";
  // import SampleImage from "$assets/img/placeholder-hero.jpg?format=webp&w=750;900;1200;2400&q=85&as=metadata";
  import IndexImage_L0 from "$assets/series/index-page/cover_l0.png?format=webp&w=1200;1800;2400&q=85&as=metadata";
  import IndexImage_L1 from "$assets/series/index-page/cover_l1.png?format=webp&w=1800;2400;3000&q=85&as=metadata";
  import BaseImage_1 from "$assets/img/profile.jpg?format=webp&w=700;1200&q=85&as=metadata";
  import BaseImage_2 from "$assets/img/pepper_dayly.jpg?format=webp&w=700;1200&q=85&as=metadata";
  import BaseImage_3 from "$assets/img/centaline_groupmate.jpg?format=webp&w=700;1200&q=85&as=metadata";

  import KeyboardImage from "$assets/img/keyboard.jpg?format=webp&q=85";
  import ScrollFrame from "$lib/scroll-frame.svelte";
  import SkillTree from "$lib/skill-tree/elk/main.svelte";
  // import { Picture } from "svelte-lazy-loader";
  import ImageSet from "$lib/Image.svelte";
  // import Card from "$lib/skill-tree/elk/card.svelte";

  const testArray = Array.from({ length: 30 }, (_, i) => i + 1);
  // console.log(IndexImage);
</script>

<svelte:head>
  <title>Steven, the web dev</title>
  <!-- <meta /> -->
</svelte:head>
<Navbar />

<Banner
  id="index"
  isFullScreen
  containerClass="h-full grid items-center content-center"
>
  <div slot="background" class="inset-0 absolute">
    <ImageSet
      class="inset-0 absolute z-1"
      srcList={IndexImage_L0}
      forceLoad
      alt="index banner image; layer 0"
    />
    <ImageSet
      class="inset-0 absolute z-2"
      srcList={IndexImage_L1}
      forceLoad
      alt="index banner image; layer 1"
    />
  </div>
  <BannerGridCell align="left">
    <h1 class="text-light-50 text-5xl leading-loose">Steven - Cheng Ho Man</h1>
    <p class="text-light-200 leading-loose">
      Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation
    </p>
  </BannerGridCell>
</Banner>

<Banner
  id="introduction"
  class="!h-500vh w-full "
  containerClass=" grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 justify-items-stretch  grid-rows-auto grid-content-start gap-2 md:gap-y-8 lg:gap-y-32 pt-32"
>
  <ScrollFrame
    slot="background"
    class="bg-dark-100 inset-0 bg-dark-100 fixed -z-1 "
  />

  <h1
    class="grid-row-start-1 grid-col-start-2 grid-col-end-2 text-center text-light-50 text-5xl"
  >
    Who I am ...
  </h1>
  <!-- {#each testArray as i}
    <BannerGridCell align={i % 2 === 0 ? "left" : "right"} row={i} class=" ">
      <h2 class="text-light-50 text-3xl">A Man who run with code</h2>
      <span>{i}</span>
      <p class="text-light-200">almost, but not quite,</p>
    </BannerGridCell>
  {/each} -->
  <BannerGridCell align="left" row={2}>
    <h2 class="text-light-50 text-3xl">A Man who run with code</h2>
    <p class="text-light-200">almost, but not quite,</p>
  </BannerGridCell>
  <BannerGridCell align="right" row={3}>
    <h2 class="text-light-50 text-3xl">A Man who run with code</h2>
    <p class="text-light-200">almost, but not quite,</p>
  </BannerGridCell>

  <h1
    class="grid-row-start-4 grid-col-start-2 grid-col-end-2 text-center text-light-50 text-5xl"
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
        <li>
          <p>Teksbotics (Hong Kong ) Limited</p>
        </li>
        <li>
          <p>Start-up Company for Robotic solution</p>
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
        <li>
          <p>Centaline Data Technology Limited</p>
        </li>
        <li>
          <p>Large-scale Property Agency Company</p>
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

  <h1
    class="grid-row-start-9 grid-col-start-2 grid-col-end-2 text-center text-light-50 text-5xl"
  >
    Hobby...
  </h1>
  <BannerGridCell align="left" row={10}>
    <h2 class="text-light-50 text-3xl">Drawing</h2>
    <div class="text-light-50 text-lg">
      just a little bit, not much, but I like it
      <div class="grid w-24 h-24"></div>
    </div>
  </BannerGridCell>
  <BannerGridCell align="right" row={11}>
    <h2 class="text-light-50 text-3xl">Program making</h2>
    <div class="text-light-50 text-lg">
      make some small util tools for myself, to make my life easier; but not a big fans of code leeting
      <div class="grid w-24 h-24"></div>
    </div>
  </BannerGridCell>
  <BannerGridCell align="left" row={12}>
    <h2 class="text-light-50 text-3xl">Game making</h2>
    <div class="text-light-50 text-lg">
      make some small util tools for myself, to make my life easier
      <div class="grid w-24 h-24"></div>
    </div>
  </BannerGridCell>
  <BannerGridCell align="right" row={13}>
    <h2 class="text-light-50 text-3xl">Keyboard</h2>
    <div class="text-light-50 text-lg">
      nice to type on, nice to look at. speed up my typing speed; also, make for
      cool stuff
    </div>
    <img
      class="mt-5 w-full min-h-24 object-contain"
      src={KeyboardImage}
      alt=""
    />
  </BannerGridCell>

  <BannerGridCell align="left" row={14}>
    <h2 class="text-light-50 text-3xl">Coffee(?)</h2>
    <div class="text-light-50 text-lg">coffee energy, enpower me everyday</div>
    <img
      class="mt-5 w-full min-h-24 object-contain"
      src="$assets/img/pepper_dayly.jpg"
      alt=""
    />
  </BannerGridCell>
</Banner>

<Banner id="skill" isFullScreen containerClass="!max-w-99rem">
  <div slot="background" class=" backdrop-blur-lg inset-0 absolute" />
  <div class="flex flex-col self-center max-w-full items-start">
    <div class="px-6 py-4 rounded-lg bg-dark-700/45 mx-8 mb-3">
      <h1 class="text-3xl text-light-900">Skill Set</h1>
      <p class="text-light-800">
        Those I known and those I am learning, all skill and tech are mapped on
        the canvas
      </p>
    </div>
    <SkillTree />
  </div>
</Banner>

<Banner id="contact" containerClass="">
  <div slot="background" class=" bg-blue-900 flex flex-col min-h-150" />

  <div class="flex flex-col">
    <h1 class="text-center text-3xl text-light-50 font-light">Contact Me</h1>
    <div class="mt-8 max-w-xl flex flex-col w-full self-center">
      <table class="table-auto">
        <tbody>
          <tr class=" border-b-1 border-white">
            <td class="py-3 text-2xl text-light-50 font-light">Email:</td>
            <td class="py-3 flex justify-end">
              <a
                class="text-right text-2xl text-light-50 font-light"
                href="mailto:st.ch.inwork@gmail.com"
              >
                st.ch.inwork@gmail.com
              </a>
            </td>
          </tr>

          <tr class=" border-b-1 border-white">
            <td class="py-3 flex flex-col">
              <span class="text-2xl text-light-50 font-light">Tel:</span>
            </td>
            <td class="py-3">
              <div class="flex flex-col">
                <a
                  class="text-right text-2xl text-light-50 font-light"
                  href="tel:+85297915541"
                >
                  +852-97915541
                </a>
                <a
                  class="text-right text-2xl text-light-50 font-light"
                  href="tel:+86435397917"
                >
                  +86-435397917
                </a>
              </div>
            </td>
          </tr>

          <tr class=" border-b-1 border-white">
            <td class="py-3 flex flex-col">
              <span class="text-2xl text-light-50 font-light">LinkedIn:</span>
            </td>
            <td class="py-3">
              <div class="flex flex-col">
                <a
                  class="text-right text-2xl text-light-50 font-light"
                  href="https://www.linkedin.com/in/chengsteven430431182/"
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
