export async function boot(init, status) {
  try {
    await init();
  } catch (error) {
    // winit 0.30 transfers control to the browser event loop by throwing.
    // Only its exact pair of markers denotes a successful handoff.
    const message = String(error?.message ?? error);
    if (!(message.includes('Using exceptions for control flow') &&
          message.includes("This isn't actually an error"))) {
      status.textContent = `Demarrage impossible : ${message}`;
      throw error;
    }
  }
  status.textContent = 'Fleches : barre / voiles · Echap : pause · R : recommencer';
}
