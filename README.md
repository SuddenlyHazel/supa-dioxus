# Supa Dioxus
> Supabase + Dioxus rapid hack stack

## Setup 
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Create a `.env` file with the following 

```
APP_PUBLIC_SUPABASE_URL=<Your supabase url>
APP_PUBLIC_ID=<Your supabase ref-if>
APP_PUBLIC_SUPABASE_ANON_KEY=<Your supabase anon key>
DB_URI=<The connection uri for supabase project>
```

3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Launch the Dioxus Fullstack app:

```bash
dx serve --platform fullstack
```