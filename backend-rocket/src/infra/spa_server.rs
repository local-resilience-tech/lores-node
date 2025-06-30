use rocket::fs::{NamedFile, Options};
use rocket::http::{Method, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::Request;
use rocket::response::Responder;
use rocket::route::{Handler, Outcome};
use rocket::{figment, yansi, Data, Route};
use std::path::{Path, PathBuf};

/// This is a custom handler for serving single page applications (SPA).
/// Heavily based on the SpaServer handler from Rocket.

/// Custom handler for serving static files.
///
/// This handler makes it simple to serve static files from a directory on the
/// local file system. To use it, construct a `SpaServer` using either
/// [`SpaServer::from()`] or [`SpaServer::new()`] then simply `mount` the
/// handler at a desired path. When mounted, the handler will generate route(s)
/// that serve the desired static files. If a requested file is not found, the
/// routes _forward_ the incoming request. The default rank of the generated
/// routes is `10`. To customize route ranking, use the [`SpaServer::rank()`]
/// method.
///
/// # Options
///
/// The handler's functionality can be customized by passing an [`Options`] to
/// [`SpaServer::new()`].
///
/// # Example
///
/// To serve files from the `/static` directory on the local file system at the
/// `/public` path, allowing `index.html` files to be used to respond to
/// requests for a directory (the default), you might write the following:
///
/// ```rust,no_run
/// # #[macro_use] extern crate rocket;
/// use rocket::fs::SpaServer;
///
/// #[launch]
/// fn rocket() -> _ {
///     rocket::build().mount("/public", SpaServer::from("/static"))
/// }
/// ```
///
/// With this, requests for files at `/public/<path..>` will be handled by
/// returning the contents of `/static/<path..>`. Requests for _directories_ at
/// `/public/<directory>` will be handled by returning the contents of
/// `/static/<directory>/index.html`.
///
/// ## Relative Paths
///
/// In the example above, `/static` is an absolute path. If your static files
/// are stored relative to your crate and your project is managed by Rocket, use
/// the [`relative!`] macro to obtain a path that is relative to your
/// crate's root. For example, to serve files in the `static` subdirectory of
/// your crate at `/`, you might write:
///
/// ```rust,no_run
/// # #[macro_use] extern crate rocket;
/// use rocket::fs::{SpaServer, relative};
///
/// #[launch]
/// fn rocket() -> _ {
///     rocket::build().mount("/", SpaServer::from(relative!("static")))
/// }
/// ```
#[derive(Debug, Clone)]
pub struct SpaServer {
    root: PathBuf,
    options: Options,
    rank: isize,
}

impl SpaServer {
    /// The default rank use by `SpaServer` routes.
    const DEFAULT_RANK: isize = 10;

    /// Constructs a new `SpaServer` that serves files from the file system
    /// `path`. By default, [`Options::Index`] is set, and the generated routes
    /// have a rank of `10`. To serve static files with other options, use
    /// [`SpaServer::new()`]. To choose a different rank for generated routes,
    /// use [`SpaServer::rank()`].
    ///
    /// # Panics
    ///
    /// Panics if `path` does not exist or is not a directory.
    ///
    /// # Example
    ///
    /// Serve the static files in the `/www/public` local directory on path
    /// `/static`.
    ///
    /// ```rust,no_run
    /// # #[macro_use] extern crate rocket;
    /// use rocket::fs::SpaServer;
    ///
    /// #[launch]
    /// fn rocket() -> _ {
    ///     rocket::build().mount("/static", SpaServer::from("/www/public"))
    /// }
    /// ```
    ///
    /// Exactly as before, but set the rank for generated routes to `30`.
    ///
    /// ```rust,no_run
    /// # #[macro_use] extern crate rocket;
    /// use rocket::fs::SpaServer;
    ///
    /// #[launch]
    /// fn rocket() -> _ {
    ///     rocket::build().mount("/static", SpaServer::from("/www/public").rank(30))
    /// }
    /// ```
    #[track_caller]
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        SpaServer::new(path, Options::default())
    }

    /// Constructs a new `SpaServer` that serves files from the file system
    /// `path` with `options` enabled. By default, the handler's routes have a
    /// rank of `10`. To choose a different rank, use [`SpaServer::rank()`].
    ///
    /// # Panics
    ///
    /// If [`Options::Missing`] is not set, panics if `path` does not exist or
    /// is not a directory. Otherwise does not panic.
    ///
    /// # Example
    ///
    /// Serve the static files in the `/www/public` local directory on path
    /// `/static` without serving index files or dot files. Additionally, serve
    /// the same files on `/pub` with a route rank of -1 while also serving
    /// index files and dot files.
    ///
    /// ```rust,no_run
    /// # #[macro_use] extern crate rocket;
    /// use rocket::fs::{SpaServer, Options};
    ///
    /// #[launch]
    /// fn rocket() -> _ {
    ///     let options = Options::Index | Options::DotFiles;
    ///     rocket::build()
    ///         .mount("/static", SpaServer::from("/www/public"))
    ///         .mount("/pub", SpaServer::new("/www/public", options).rank(-1))
    /// }
    /// ```
    #[track_caller]
    pub fn new<P: AsRef<Path>>(path: P, options: Options) -> Self {
        use yansi::Paint;

        let path = path.as_ref();
        if !options.contains(Options::Missing) {
            if !options.contains(Options::IndexFile) && !path.is_dir() {
                let path = path.display();
                error!("SpaServer path '{}' is not a directory.", path.primary());
                warn_!("Aborting early to prevent inevitable handler error.");
                panic!("invalid directory: refusing to continue");
            } else if !path.exists() {
                let path = path.display();
                error!("SpaServer path '{}' is not a file.", path.primary());
                warn_!("Aborting early to prevent inevitable handler error.");
                panic!("invalid file: refusing to continue");
            }
        }

        SpaServer {
            root: path.into(),
            options,
            rank: Self::DEFAULT_RANK,
        }
    }

    /// Sets the rank for generated routes to `rank`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rocket::fs::{SpaServer, Options};
    ///
    /// // A `SpaServer` created with `from()` with routes of rank `3`.
    /// SpaServer::from("/public").rank(3);
    ///
    /// // A `SpaServer` created with `new()` with routes of rank `-15`.
    /// SpaServer::new("/public", Options::Index).rank(-15);
    /// ```
    pub fn rank(mut self, rank: isize) -> Self {
        self.rank = rank;
        self
    }
}

impl From<SpaServer> for Vec<Route> {
    fn from(server: SpaServer) -> Self {
        let source = figment::Source::File(server.root.clone());
        let mut route = Route::ranked(server.rank, Method::Get, "/<path..>", server);
        route.name = Some(format!("SpaServer: {}", source).into());
        vec![route]
    }
}

#[async_trait]
impl Handler for SpaServer {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        let options = self.options;
        if options.contains(Options::IndexFile) && self.root.is_file() {
            let file = NamedFile::open(&self.root).await;
            return file.respond_to(req).or_forward((data, Status::NotFound));
        }

        Outcome::forward(data, Status::NotFound)
    }
}
