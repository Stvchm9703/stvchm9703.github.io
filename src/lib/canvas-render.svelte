<script lang="ts">
  import { onMount } from "svelte";
  import * as THREE from "three";
  import { OrbitControls } from "three/addons/controls/OrbitControls.js";
  import { ConvexGeometry } from "three/addons/geometries/ConvexGeometry.js";
  import * as BufferGeometryUtils from "three/addons/utils/BufferGeometryUtils.js";

  /** @type {HTMLCanvasElement} */
  let canvasElement;
  let lastScrollTop = 0;

  let group, camera, scene, renderer;
  function init() {
    scene = new THREE.Scene();
    renderer = new THREE.WebGLRenderer({
      canvas: canvasElement,
      context: canvasElement.getContext("webgl2"),
      antialias: true,
    });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(window.innerWidth - 16, window.innerHeight - 16);
    // document.body.appendChild(renderer.domElement);

    // camera

    camera = new THREE.PerspectiveCamera(
      40,
      window.innerWidth / window.innerHeight,
      1,
      1000
    );
    camera.position.set(15, 20, 30);
    scene.add(camera);

    // controls

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.minDistance = 50;
    controls.maxDistance = 50;
    // controls.maxPolarAngle = Math.PI / 2;

    // ambient light

    scene.add(new THREE.AmbientLight(0x222222));

    // point light

    const light = new THREE.PointLight(0xffffff, 1);
    camera.add(light);


    group = new THREE.Group();
    scene.add(group);

    // points

    let dodecahedronGeometry = new THREE.DodecahedronGeometry(5);


    dodecahedronGeometry.deleteAttribute("normal");
    dodecahedronGeometry.deleteAttribute("uv");

    dodecahedronGeometry =
      BufferGeometryUtils.mergeVertices(dodecahedronGeometry);

    const vertices = [];
    const positionAttribute = dodecahedronGeometry.getAttribute("position");

    for (let i = 0; i < positionAttribute.count; i++) {
      const vertex = new THREE.Vector3();
      vertex.fromBufferAttribute(positionAttribute, i);
      vertices.push(vertex);
    }



    const meshMaterial = new THREE.MeshLambertMaterial({
      color: 0xffffff,
      opacity: 0.5,
      side: THREE.DoubleSide,
      transparent: true,
    });

    const meshGeometry = new ConvexGeometry(vertices);

    const mesh = new THREE.Mesh(meshGeometry, meshMaterial);
    group.add(mesh);

    //

    window.addEventListener("resize", onWindowResize);
  }

  function onWindowResize() {
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize(window.innerWidth, window.innerHeight);
  }

  function animate() {
    requestAnimationFrame(animate);
    // group.rotation.y += 0.005;
    render();
  }

  function render() {
    renderer.render(scene, camera);
  }
  const renderCanvasSample = () => {
    // const ctx = canvasElement.getContext("2d");

    init();
    animate();
  };

  onMount(() => {
    console.log("mounted");
    renderCanvasSample();
    window.addEventListener("scroll", (e) => {
      let currentScroll =
        document.documentElement.scrollTop || document.body.scrollTop;
      // Get Current Scroll Value
      let delta = lastScrollTop - currentScroll;
      group.rotation.y += 0.1 * (delta > 0 ? 1 : -1);
      lastScrollTop = currentScroll;
    });
  });
</script>

<canvas bind:this={canvasElement} class={$$restProps.class || ""} />
