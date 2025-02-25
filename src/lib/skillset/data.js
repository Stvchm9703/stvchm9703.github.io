import icon_vuejs from '$assets/img/vuejs.svg'
import icon_nuxtjs from '$assets/img/nuxt-square.svg'
import icon_nodejs from '$assets/img/icon_nodejs.svg'
import icon_python from '$assets/img/icon_Python.svg'
import icon_csharp from '$assets/img/csharp.svg'
import icon_golang from '$assets/img/Go_Logo_Blue.svg'
import icon_rust from '$assets/img/rust.svg'
import icon_svelte from '$assets/img/icon_svelte.svg'
import icon_solidjs from '$assets/img/icon_solidjs.svg'
import icon_astro from '$assets/img/icon_astro.svg'
import icon_godot from '$assets/img/icon_godot.svg'
import icon_deno from '$assets/img/icon_deno.svg'
import icon_expressjs from '$assets/img/icon_expressjs.svg'
import icon_jquery from '$assets/img/icon_jquery.svg'
import icon_tailwindcss from '$assets/img/icon_tailwind-css.svg'
import icon_rollup from '$assets/img/icon_rollup.svg'
import icon_vite from '$assets/img/icon_vite.svg'
import icon_mysql from '$assets/img/icon_mysql.svg'
import icon_pgsql from '$assets/img/icon_pgsql.svg'
import icon_mongodb from '$assets/img/icon_mongodb.svg'
import icon_redis from '$assets/img/icon_redis.svg'
import icon_pytorch from '$assets/img/icon_pytorch.svg'
import icon_flask from '$assets/img/icon_flask.svg'
import icon_django from '$assets/img/icon_django.svg'
import icon_docker from '$assets/img/icon_docker.svg'
import icon_unity from '$assets/img/icon_unity.svg'
import icon_htmx from '$assets/img/icon_htmx.svg'
import icon_grpc from '$assets/img/icon_grpc.svg'
import icon_asp from '$assets/img/icon_asp.svg'
import icon_tauri from '$assets/img/icon_tauri.svg'
import icon_tokio from '$assets/img/icon_tokio.svg'
import icon_storybook from '$assets/img/icon_storybook.svg'
import icon_leptos from '$assets/img/icon_leptos.png'
import icon_sass from '$assets/img/icon_sass.svg'
import icon_postcss from '$assets/img/icon_postcss.svg'

// l0: 1
// l1: 5
// l2: 12
// l3: 18

// node 
const nodes = [
  // {  name: 'Terminal (start point)', layer: 0, parent: null, color: '#000000' },
  // layer 1
  // {  name: 'Frontend',  color: '#3498db' },
  // {  name: 'Backend',  color: '#2ecc71' },
  // {  name: 'Data-Analysis',  color: '#f39c12' },
  // {  name: 'DevOps',  color: '#2c3e50' },
  // {  name: 'Game Dev',  color: '#3498db' },

  // layer 2 limit-6
  {  name: 'Vue-js',  icon: icon_vuejs, color: '#41b883' },
  {  name: 'Nuxt-js',  icon: icon_nuxtjs, color: '#00c58e' },
  {  name: 'Svelte',  icon: icon_svelte, color: '#ff3e00' },
  {  name: 'Astro-js',  icon: icon_astro, color: '#000000' },
  // {  name: 'others',  color: '#607d8b' },

  // layer 3 limit-9
  {  name: 'Node-js',  icon: icon_nodejs, color: '#68a063' },
  {  name: 'Solid-js',  icon: icon_solidjs, color: '#4377bb' },
  {  name: 'Express-js',  icon: icon_expressjs, color: '#000000' },
  {  name: 'Deno',  icon: icon_deno, color: '#000000' },
  {  name: 'HTMX',  icon: icon_htmx, color: '#1a2b34' },
  {  name: 'jQuery',  icon: icon_jquery, color: '#0769ad' },

  // layer 4 limit-12
  {  name: 'RollupJs',  icon: icon_rollup, color: '#ec4a3f' },
  {  name: 'ViteJs',  icon: icon_vite, color: '#646c70' },
  {  name: 'Design System',  icon: icon_storybook, color: '#ff4785' },
  {  name: 'Tailwind CSS',  icon: icon_tailwindcss, color: '#38b2ac' },
  {  name: 'SCSS/SASS',  icon: icon_sass, color: '#cc6699' },
  {  name: 'PostCSS',  icon: icon_postcss, color: '#dd3a0a' },
  {  name: 'Tauri.app',  icon: icon_tauri, color: '#24c8db' },

  // layer 2 limit-6 
  {  name: 'CSharp',  icon: icon_csharp, color: '#239120' },
  {  name: 'Golang',  icon: icon_golang, color: '#00acd7' },
  {  name: 'Rust',  icon: icon_rust, color: '#dea584' },
  // {  name: 'Database',  color: '#4479a1' },
  {  name: 'Python',  icon: icon_python, color: '#306998' },

  // layer 3 limit-9
  {  name: 'PostgreSQL',  icon: icon_pgsql, color: '#336791' },
  {  name: 'MySQL',  icon: icon_mysql, color: '#4479a1' },
  {  name: 'MongoDB',  icon: icon_mongodb, color: '#13aa52' },
  {  name: 'Redis',  icon: icon_redis, color: '#d82c20' },

  {  name: 'Pytorch',  icon: icon_pytorch, color: '#ee4c2c' },
  {  name: 'Django',  icon: icon_django, color: '#092e20' },
  {  name: 'Flask',  icon: icon_flask, color: '#306998' },

  {  name: 'Docker',  icon: icon_docker, color: '#2496ed' },
  // layer 4 limit-12
  {  name: 'Actrix',  color: '#3386f2' },
  {  name: 'Axum',  icon: icon_tokio, color: '#00599c' },
  {  name: 'Tonic',  icon: icon_grpc, color: '#754425' },
  {  name: 'Leptos',  icon: icon_leptos, color: '#dea584' },

  {  name: 'Unity',  icon: icon_unity, color: '#000000' },
  {  name: 'Godot',  icon: icon_godot, color: '#478cbf' },

  {  name: 'Gin',  color: '#00acd7' },
  {  name: 'gRPC',  icon: icon_grpc, color: '#76a5af' },

  {  name: 'ASP.NET',  icon: icon_asp, color: '#5c2d91' },
]
.map((node, id) => ({ ...node, id }))

export {
  // layer,
  nodes
}