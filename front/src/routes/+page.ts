export const prerender = true;
export const ssr = false;

type Sim = {
  name: string
  stats: Array<[string, number]>
}

export async function load( ) : Promise<any> {
  let saving = false;

  return {
    sim: {
      name: "Bruna",
      stats: Object.entries({
        energy: 50,
        hunger: 25, 
        fun: 10,
        social: 64,
        hygiene: 80
      })
    } as Sim,
    saveData: async (data: Sim) => {
      // Create delay to only save after 1 second without changes
      if (saving) return;
      saving = true;
      await new Promise(resolve => setTimeout(resolve, 1000)).
        then(() => {
             console.log('Saved');
             saving = false
      });

      console.log(data)
    }
  }
}
