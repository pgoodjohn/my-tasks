/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file was automatically generated by TanStack Router.
// You should NOT make any changes in this file as it will be overwritten.
// Additionally, you should also exclude this file from your linter and/or formatter to prevent it from being checked or modified.

// Import Routes

import { Route as rootRoute } from './routes/__root'
import { Route as SettingsRouteImport } from './routes/settings.route'
import { Route as IndexRouteImport } from './routes/index.route'
import { Route as TasksIndexRouteImport } from './routes/tasks/index.route'
import { Route as ProjectsIndexRouteImport } from './routes/projects/index.route'
import { Route as ProjectsProjectIdRouteImport } from './routes/projects/$projectId.route'

// Create/Update Routes

const SettingsRouteRoute = SettingsRouteImport.update({
  id: '/settings',
  path: '/settings',
  getParentRoute: () => rootRoute,
} as any)

const IndexRouteRoute = IndexRouteImport.update({
  id: '/',
  path: '/',
  getParentRoute: () => rootRoute,
} as any)

const TasksIndexRouteRoute = TasksIndexRouteImport.update({
  id: '/tasks/',
  path: '/tasks/',
  getParentRoute: () => rootRoute,
} as any)

const ProjectsIndexRouteRoute = ProjectsIndexRouteImport.update({
  id: '/projects/',
  path: '/projects/',
  getParentRoute: () => rootRoute,
} as any)

const ProjectsProjectIdRouteRoute = ProjectsProjectIdRouteImport.update({
  id: '/projects/$projectId',
  path: '/projects/$projectId',
  getParentRoute: () => rootRoute,
} as any)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      id: '/'
      path: '/'
      fullPath: '/'
      preLoaderRoute: typeof IndexRouteImport
      parentRoute: typeof rootRoute
    }
    '/settings': {
      id: '/settings'
      path: '/settings'
      fullPath: '/settings'
      preLoaderRoute: typeof SettingsRouteImport
      parentRoute: typeof rootRoute
    }
    '/projects/$projectId': {
      id: '/projects/$projectId'
      path: '/projects/$projectId'
      fullPath: '/projects/$projectId'
      preLoaderRoute: typeof ProjectsProjectIdRouteImport
      parentRoute: typeof rootRoute
    }
    '/projects/': {
      id: '/projects/'
      path: '/projects'
      fullPath: '/projects'
      preLoaderRoute: typeof ProjectsIndexRouteImport
      parentRoute: typeof rootRoute
    }
    '/tasks/': {
      id: '/tasks/'
      path: '/tasks'
      fullPath: '/tasks'
      preLoaderRoute: typeof TasksIndexRouteImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export interface FileRoutesByFullPath {
  '/': typeof IndexRouteRoute
  '/settings': typeof SettingsRouteRoute
  '/projects/$projectId': typeof ProjectsProjectIdRouteRoute
  '/projects': typeof ProjectsIndexRouteRoute
  '/tasks': typeof TasksIndexRouteRoute
}

export interface FileRoutesByTo {
  '/': typeof IndexRouteRoute
  '/settings': typeof SettingsRouteRoute
  '/projects/$projectId': typeof ProjectsProjectIdRouteRoute
  '/projects': typeof ProjectsIndexRouteRoute
  '/tasks': typeof TasksIndexRouteRoute
}

export interface FileRoutesById {
  __root__: typeof rootRoute
  '/': typeof IndexRouteRoute
  '/settings': typeof SettingsRouteRoute
  '/projects/$projectId': typeof ProjectsProjectIdRouteRoute
  '/projects/': typeof ProjectsIndexRouteRoute
  '/tasks/': typeof TasksIndexRouteRoute
}

export interface FileRouteTypes {
  fileRoutesByFullPath: FileRoutesByFullPath
  fullPaths: '/' | '/settings' | '/projects/$projectId' | '/projects' | '/tasks'
  fileRoutesByTo: FileRoutesByTo
  to: '/' | '/settings' | '/projects/$projectId' | '/projects' | '/tasks'
  id:
    | '__root__'
    | '/'
    | '/settings'
    | '/projects/$projectId'
    | '/projects/'
    | '/tasks/'
  fileRoutesById: FileRoutesById
}

export interface RootRouteChildren {
  IndexRouteRoute: typeof IndexRouteRoute
  SettingsRouteRoute: typeof SettingsRouteRoute
  ProjectsProjectIdRouteRoute: typeof ProjectsProjectIdRouteRoute
  ProjectsIndexRouteRoute: typeof ProjectsIndexRouteRoute
  TasksIndexRouteRoute: typeof TasksIndexRouteRoute
}

const rootRouteChildren: RootRouteChildren = {
  IndexRouteRoute: IndexRouteRoute,
  SettingsRouteRoute: SettingsRouteRoute,
  ProjectsProjectIdRouteRoute: ProjectsProjectIdRouteRoute,
  ProjectsIndexRouteRoute: ProjectsIndexRouteRoute,
  TasksIndexRouteRoute: TasksIndexRouteRoute,
}

export const routeTree = rootRoute
  ._addFileChildren(rootRouteChildren)
  ._addFileTypes<FileRouteTypes>()

/* ROUTE_MANIFEST_START
{
  "routes": {
    "__root__": {
      "filePath": "__root.tsx",
      "children": [
        "/",
        "/settings",
        "/projects/$projectId",
        "/projects/",
        "/tasks/"
      ]
    },
    "/": {
      "filePath": "index.route.tsx"
    },
    "/settings": {
      "filePath": "settings.route.tsx"
    },
    "/projects/$projectId": {
      "filePath": "projects/$projectId.route.tsx"
    },
    "/projects/": {
      "filePath": "projects/index.route.tsx"
    },
    "/tasks/": {
      "filePath": "tasks/index.route.tsx"
    }
  }
}
ROUTE_MANIFEST_END */
