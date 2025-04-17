export const prerender = true;
export const ssr = false;

type Sim = {
  name: string
  stats: Array<{
    name: string, 
    value: string
  }>
}

export async function load({fetch} ) : Promise<any> {
  let saving = false;

  console.log(import.meta.env.VITE_API_ENDPOINT)
  let req = await fetch("http://" + import.meta.env.VITE_API_ENDPOINT + '/api/v1/sim')
  let sim = await req.json()

  return {
    sim: sim as Sim,
    saveData: async (data: Sim) => {
      // Create delay to only save after 1 second without changes
      if (saving) return;
      saving = true;
      await new Promise(resolve => setTimeout(resolve, 1000)).
        then(() => {
             console.log(JSON.stringify(data))
             let req = fetch("http://" + import.meta.env.VITE_API_ENDPOINT + '/api/v1/sim', {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json'
              },
              body: JSON.stringify(data)
             })

             saving = false
      });

    }
  }
}
