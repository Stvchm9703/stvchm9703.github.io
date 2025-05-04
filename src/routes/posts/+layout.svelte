<script>

  import "uno.css";

    import { page } from "$app/state";

    // let today_date = new Date().toLocaleDateString();
    const { today_date, today_year  } = page.data;
    const { children } = $props();

    import { MetaTags } from 'svelte-meta-tags';
</script>


<svelte:head>
    <meta name="author" content="Steven, Cheng Ho Man" />
</svelte:head>

<MetaTags
    title="Steven Dev;s Log"
    description="Developer blog, Steven Dev;s Log"
/>

<div class="container mx-auto px-4  max-w-7xl">
    <section class="pt-4 pb-2 lg:pb-4">
        <!-- {/* Newspaper Header */} -->
        <div class="border-b-2 border-dark-900 lg:pb-4 lg:mb-8">
            <h1
                class="font-serif text-5xl md:text-7xl text-center font-bold tracking-tight"
            >
                Steven Dev;s Log
            </h1>
            <div class="flex justify-between items-center mt-2 text-sm">
                <p>Vol. 123, No. 45</p>
                <p>{today_date}</p>
                <!-- <p>Est. 1902</p> -->
            </div>
        </div>
    </section>

    {@render children()}

    <section class="py-4">
        <!-- {/* Footer */} -->
        <footer class="border-t pt-4 pb-6 lg:pt-8 lg:pb-12">
            <div
                class="text-center text-sm text-muted-foreground"
            >
                <p>© {today_year} Ho Man Cheng, Steven. All rights reserved.</p>
                <div class="flex justify-center gap-4 mt-2 print:hidden">
                    <a href="/terms" class="hover:underline"> Terms of Service </a>
                    <a href="/privacy" class="hover:underline"> Privacy Policy </a>
                    <a href="/contact" class="hover:underline"> Contact Us </a>
                </div>
            </div>
        </footer>
    </section>
</div>
