// const clerkFrontendApi = "pk_test_bWFqb3ItamF5YmlyZC03LmNsZXJrLmFjY291bnRzLmRldiQ"; // Replace this
//
// window.Clerk.load({
//   frontendApi: clerkFrontendApi,
//   publishableKey: clerkFrontendApi,
// });
//
// window.Clerk.mountSignInButton("#auth-buttons", {
//   afterSignInUrl: "/",
// });
//
// window.Clerk.mountUserButton("#auth-buttons");
//
// document
//   .getElementById("call-protected")
//   .addEventListener("click", async () => {
//     const token = await window.Clerk.session.getToken();
//
//     const res = await fetch("/api/protected", {
//       method: "POST",
//       headers: {
//         Authorization: `Bearer ${token}`,
//       },
//     });
//
//     const text = await res.text();
//     document.getElementById("result").textContent = text;
//   });
