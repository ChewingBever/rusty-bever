# API Design

This file describes the API that the software adheres to. All routes are defined under a shared `api` namespace.

`(A)` means the route can only be accessed by an admin user.

## v1

## Authentification

* POST `/auth/login` - generate new JWT & refresh token pair given user credentials
* POST `/auth/refresh` - generate new JWT & refresh token pair given valid refresh token

## Posts

* GET `/posts?<offset>&<limit>` - get list of posts from the default feed given offset & limit
* GET `/posts?<section_id_or_shortname>&<offset>&<limit>` - get list of posts of a specific section
* (A) POST `/posts` - create a new post
* GET `/posts/<id_or_shortname>` - get a specific post
* (A) DELETE `/posts/<id_or_shortname>` - delete a post
* (A) PATCH `/posts/<id_or_shortname>` - patch a post

## Sections

* GET `/sections?<offset>&<limit>` - get list of sections
* GET `/sections/<id_or_shortname>` - get specific section
* (A) POST `/sections` - create a new section
* (A) PATCH `/sections/<id_or_shortname>` - patch a section
* (A) DELETE `/sections/<id_or_shortname>` - delete a section (what happens with posts?)

## Users

* (A) GET `/users?<offset>&<limit>`
* (A) POST `/users`
* (A) GET `/users/<id_or_username>`
* (A) PATCH `/users/<id_or_username>`
* (A) DELETE `/users/<id_or_username>`

## Feeds

WIP
