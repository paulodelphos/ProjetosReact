import { initializeApp } from 'firebase/app'
import { getStorage } from 'firebase/storage'

//const firebaseConfig = {
//    apiKey: process.env.REACT_APP_FIREBASE_APIKEY,
//    authDomain: process.env.REACT_APP_FIREBASE_AUTODOMAIN,
//    projectId: process.env.REACT_APP_FIREBASE_PROJECTID,
//    storageBucket: process.env.REACT_APP_FIREBASE_STORAGEBUCKET,
//    messagingSenderId: process.env.REACT_APP_FIREBASE_MESSAGINGSENDERID,
//    appId: process.env.REACT_APP_FIREBASE_APPID
//  };
// Your web app's Firebase configuration
const firebaseConfig = {
  apiKey: "AIzaSyC02IBPubxK0vBT0nIhw-KM6adCjw8FBy0",
  authDomain: "reactgaleria.firebaseapp.com",
  projectId: "reactgaleria",
  storageBucket: "reactgaleria.appspot.com",
  messagingSenderId: "528356183157",
  appId: "1:528356183157:web:b939297737f65545577c3c"
};


const firebaseApp = initializeApp(firebaseConfig);

export const storage = getStorage(firebaseApp);