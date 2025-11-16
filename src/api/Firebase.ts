import { FirebaseApp, initializeApp } from "firebase/app";
import { Analytics, getAnalytics, logEvent } from "firebase/analytics";
import { Auth, getAuth, signInWithCustomToken } from "firebase/auth";
import {
  getFirestore,
  getDoc,
  updateDoc,
  Firestore,
  doc,
} from "firebase/firestore/lite";

const firebaseConfig = {
  apiKey: "AIzaSyDR_xgBK2-HdpNAaF_92CxSwezZxAOUnNs",
  authDomain: "kiwi-app-7dbb6.firebaseapp.com",
  projectId: "kiwi-app-7dbb6",
  storageBucket: "kiwi-app-7dbb6.firebasestorage.app",
  messagingSenderId: "596016300611",
  appId: "1:596016300611:web:2f10ef1c8293d5995e4af6",
  measurementId: "G-WKPJWZ2EF2",
};

class Firebase {
  app: FirebaseApp;
  auth: Auth;

  constructor() {
    this.app = initializeApp(firebaseConfig);
    this.auth = getAuth(this.app);
  }

  async signInWithToken(token: string) {
    await signInWithCustomToken(this.auth, token);
  }

  getAnalytics(): FirebaseAnalytics {
    return new FirebaseAnalytics(this.app);
  }

  getFirestore(): FirebaseFirestore {
    return new FirebaseFirestore(this.app, this.auth);
  }
}

class FirebaseAnalytics {
  private app: FirebaseApp;
  private analytics: Analytics;
  constructor(app: FirebaseApp) {
    this.app = app;
    this.analytics = getAnalytics(this.app);
  }

  /**
   * 上报事件
   * @param eventName 事件名称
   * @param eventParams 事件参数
   */
  logEvent(eventName: string, eventParams?: Record<string, any>) {
    logEvent(this.analytics, eventName, eventParams);
  }
}

class FirebaseFirestore {
  private app: FirebaseApp;
  private db: Firestore;
  private collection: string | undefined;
  private auth: Auth;

  constructor(app: FirebaseApp, auth: Auth) {
    this.app = app;
    this.auth = auth;
    this.db = getFirestore(this.app);
  }

  user(): FirebaseFirestore {
    this.collection = "user";
    return this;
  }

  async set(documentId: string, value: object): Promise<void> {
    if (!this.collection) {
      throw new Error("Collection is not defined.");
    }
    if (!this.auth.currentUser) {
      throw new Error(FirebaseErrorCode.NotLoggedIn);
    }
    const document = doc(this.db, this.collection);
    await updateDoc(document, documentId, value);
  }

  async get(documentId: string): Promise<object | null> {
    if (!this.collection) {
      throw new Error("Collection is not defined.");
    }
    if (!this.auth.currentUser) {
      throw new Error(FirebaseErrorCode.NotLoggedIn);
    }
    const docRef = doc(this.db, this.collection, documentId);
    const docSnap = await getDoc(docRef);
    if (docSnap.exists()) {
      return docSnap.data();
    } else {
      return null;
    }
  }
}

const firebase = new Firebase();
export default firebase;

/**
 * Firebase 相关错误码
 * 用于对外表示需要重新登录的错误
 */
export enum FirebaseErrorCode {
  NotLoggedIn = "notlogin",
}
