+++
title = "Ionic Mobile"
date = "2022-12-10"
tags = ["ionic", "mobile-development", "angular"]
excerpt = "Building cross-platform mobile apps with Ionic, Capacitor, and Angular. Covers navigation, native features, and app store deployment."
+++

Ionic is a framework for building cross-platform mobile applications using web technologies. Paired with Capacitor, it provides access to native device features through a JavaScript bridge.

## Why Ionic?

- **Single codebase** 鈥?one codebase for iOS, Android, and web
- **Web technology** 鈥?use HTML, CSS, and TypeScript
- **Capacitor** 鈥?modern native runtime with plugin ecosystem
- **Massive ecosystem** 鈥?Angular, React, or Vue support

## Project Setup

```bash
npm install -g @ionic/cli
ionic start my-app tabs --type=angular
cd my-app
ionic serve
```

## Component Library

Ionic provides a comprehensive set of UI components that look native on each platform:

```html
<ion-header>
  <ion-toolbar>
    <ion-title>My App</ion-title>
  </ion-toolbar>
</ion-header>

<ion-content>
  <ion-list>
    <ion-item>
      <ion-avatar slot="start">
        <img src="avatar.jpg" alt="avatar" />
      </ion-avatar>
      <ion-label>
        <h2>John Doe</h2>
        <p>Online</p>
      </ion-label>
    </ion-item>
  </ion-list>

  <ion-fab vertical="bottom" horizontal="end" slot="fixed">
    <ion-fab-button>
      <ion-icon name="add"></ion-icon>
    </ion-fab-button>
  </ion-fab>
</ion-content>
```

## Navigation with Angular Router

Ionic integrates seamlessly with Angular Router:

```typescript
const routes: Routes = [
  {
    path: 'tabs',
    loadChildren: () => import('./tabs/tabs.module').then(m => m.TabsPageModule)
  },
  {
    path: 'detail/:id',
    loadChildren: () => import('./detail/detail.module').then(m => m.DetailPageModule)
  },
  {
    path: '',
    redirectTo: '/tabs/home',
    pathMatch: 'full'
  }
];
```

## Native Features with Capacitor

```typescript
import { Camera, CameraResultType } from '@capacitor/camera';
import { Geolocation } from '@capacitor/geolocation';

async function takePicture() {
  const image = await Camera.getPhoto({
    quality: 90,
    allowEditing: true,
    resultType: CameraResultType.Uri
  });
  return image.webPath;
}

async function getCurrentPosition() {
  const coordinates = await Geolocation.getCurrentPosition();
  return {
    lat: coordinates.coords.latitude,
    lng: coordinates.coords.longitude
  };
}
```

## Building for Production

```bash
# Build for iOS
ionic build
npx cap add ios
npx cap open ios

# Build for Android
ionic build
npx cap add android
npx cap open android

# Sync web code to native projects
npx cap sync
```

## App Store Deployment Checklist

- [ ] Test on real devices
- [ ] Set app icons and splash screens
- [ ] Configure app signing
- [ ] Update version numbers
- [ ] Prepare store screenshots
- [ ] Write privacy policy

```json
// capacitor.config.json
{
  "appId": "com.example.myapp",
  "appName": "MyApp",
  "webDir": "www",
  "server": {
    "androidScheme": "https"
  },
  "plugins": {
    "SplashScreen": {
      "launchShowDuration": 3000
    }
  }
}
```

## Performance Optimization

- Use lazy loading for feature modules
- Optimize images before bundling
- Enable Ahead-of-Time (AOT) compilation
- Use Ionic's virtual scroll for long lists

```html
<ion-virtual-scroll [items]="items">
  <ion-item *virtualItem="let item">
    <ion-label>{{ item.name }}</ion-label>
  </ion-item>
</ion-virtual-scroll>
```

Ionic bridges the gap between web development speed and native mobile capabilities.
