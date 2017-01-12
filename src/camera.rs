use na::{self, Norm, Inverse, Matrix4, ToHomogeneous};

use {Vector3f, Point3f, Point2f, Transform};
use bounds::Bounds2f;
use ray::{Ray, RayDifferential};
use sampling;

/// Projective pinhole camera.
/// TODO: abstract the Camera interface and handle multiple camera types.
#[derive(Debug)]
pub struct Camera {
    camera_to_world: Transform,
    camera_to_screen: Matrix4<f32>,
    raster_to_camera: Matrix4<f32>,
    lens_radius: f32,
    focal_distance: f32,
    dx_camera: Vector3f,
    dy_camera: Vector3f,
}

impl Camera {
    pub fn new(camera_to_world: Transform,
               film_size: Point2f,
               lens_radius: f32,
               focal_distance: f32,
               fov: f32)
               -> Camera {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn scale(x: f32, y: f32, z: f32) -> Matrix4<f32> {
            Matrix4::<f32>::new(x, 0.0, 0.0, 0.0,
                                0.0, y, 0.0, 0.0,
                                0.0, 0.0, z, 0.0,
                                0.0, 0.0, 0.0, 1.0)
        }

        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn translate(x: f32, y: f32, z: f32) -> Matrix4<f32> {
            Matrix4::<f32>::new(1.0, 0.0, 0.0, x,
                                0.0, 1.0, 0.0, y,
                                0.0, 0.0, 1.0, z,
                                0.0, 0.0, 0.0, 1.0)
        }

        #[cfg_attr(rustfmt, rustfmt_skip)]
        fn perspective(fov: f32, n: f32, f: f32) -> Matrix4<f32> {
            let persp = Matrix4::<f32>::new(1.0, 0.0, 0.0, 0.0,
                                            0.0, 1.0, 0.0, 0.0,
                                            0.0, 0.0, f / (f - n), -f * n / (f - n),
                                            0.0, 0.0, 1.0, 0.0);
            let inv_tan_ang = 1.0 / (fov.to_radians() / 2.0).tan();
            scale(inv_tan_ang, inv_tan_ang, 1.0) * persp
        }

        let aspect_ratio = film_size.x / film_size.y;
        let screen_window = if aspect_ratio > 1.0 {
            Bounds2f::from_points(&Point2f::new(-aspect_ratio, -1.0),
                                  &Point2f::new(aspect_ratio, 1.0))
        } else {
            Bounds2f::from_points(&Point2f::new(-1.0, -1.0 / aspect_ratio),
                                  &Point2f::new(1.0, 1.0 / aspect_ratio))
        };
        let camera_to_screen = perspective(fov, 1e-2, 1000.0);
        let screen_to_raster = scale(film_size.x, film_size.y, 1.0) *
                               scale(1.0 / (screen_window.p_max.x - screen_window.p_min.x),
                                     1.0 / (screen_window.p_min.y - screen_window.p_max.y),
                                     1.0) *
                               translate(-screen_window.p_min.x, -screen_window.p_max.y, 0.0);
        let raster_to_screen = screen_to_raster.inverse().unwrap();
        let raster_to_camera = camera_to_screen.inverse().unwrap() * raster_to_screen;

        // compute differential changes in origin for perspective camera rays
        let dx_camera = na::from_homogeneous(&((raster_to_camera *
                                                Point3f::new(1.0, 0.0, 0.0).to_homogeneous()) -
                                               (raster_to_camera *
                                                Point3f::new(0.0, 0.0, 0.0).to_homogeneous())));
        let dy_camera = na::from_homogeneous(&((raster_to_camera *
                                                Point3f::new(0.0, 1.0, 0.0).to_homogeneous()) -
                                               (raster_to_camera *
                                                Point3f::new(0.0, 0.0, 0.0).to_homogeneous())));

        Camera {
            camera_to_world: camera_to_world,
            camera_to_screen: camera_to_screen,
            raster_to_camera: raster_to_camera,
            lens_radius: lens_radius,
            focal_distance: focal_distance,
            dx_camera: dx_camera,
            dy_camera: dy_camera,
        }
    }

    pub fn generate_ray(&self, sample: &CameraSample) -> Ray {
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera: Point3f = na::from_homogeneous(&(self.raster_to_camera *
                                                       p_film.to_homogeneous()));

        let mut ray = Ray::new(na::origin(), p_camera.to_vector().normalize());
        // modify ray for depth of field
        if self.lens_radius > 0.0 {
            // Sample point on lens
            let p_lens = self.lens_radius * sampling::concentric_sample_disk(&sample.p_lens);
            // Compute point on plane of focus
            let ft = self.focal_distance / ray.d.z;
            let p_focus = ray.at(ft);
            // Update ray for effect of lens
            ray.o = Point3f::new(p_lens.x, p_lens.y, 0.0);
            ray.d = (p_focus - ray.o).normalize();
        }
        ray.transform(&self.camera_to_world).0
    }

    pub fn generate_ray_differential(&self, sample: &CameraSample) -> Ray {
        let p_film = Point3f::new(sample.p_film.x, sample.p_film.y, 0.0);
        let p_camera: Point3f = na::from_homogeneous(&(self.raster_to_camera *
                                                       p_film.to_homogeneous()));

        let mut ray = Ray::new(na::origin(), p_camera.to_vector().normalize());
        // modify ray for depth of field
        if self.lens_radius > 0.0 {
            // Sample point on lens
            let p_lens = self.lens_radius * sampling::concentric_sample_disk(&sample.p_lens);
            // Compute point on plane of focus
            let ft = self.focal_distance / ray.d.z;
            let p_focus = ray.at(ft);
            // Update ray for effect of lens
            ray.o = Point3f::new(p_lens.x, p_lens.y, 0.0);
            ray.d = (p_focus - ray.o).normalize();
        }
        // compute offset rays for PerspectiveCamera ray differentials
        let diff = if self.lens_radius > 0.0 {
            // Sample point on lens
            let p_lens = self.lens_radius * sampling::concentric_sample_disk(&sample.p_lens);
            let origin = Point3f::new(p_lens.x, p_lens.y, 0.0);

            // ray differential in x direction
            let dx = (p_camera + self.dx_camera).to_vector().normalize();
            let ft_x = self.focal_distance / dx.z;
            let p_focus_x = (ft_x * dx).to_point();
            let rx_dir = (p_focus_x - origin).normalize();

            // ray differential in x direction
            let dy = (p_camera + self.dy_camera).to_vector().normalize();
            let ft_y = self.focal_distance / dy.z;
            let p_focus_y = (ft_y * dy).to_point();
            let ry_dir = (p_focus_y - origin).normalize();

            RayDifferential {
                rx_origin: origin,
                ry_origin: origin,
                rx_direction: rx_dir,
                ry_direction: ry_dir,
            }
        } else {
            RayDifferential {
                rx_origin: ray.o,
                ry_origin: ray.o,
                rx_direction: (p_camera.to_vector() + self.dx_camera).normalize(),
                ry_direction: (p_camera.to_vector() + self.dy_camera).normalize(),
            }
        };

        ray.differential = Some(diff);

        ray.transform(&self.camera_to_world).0
    }
}

pub struct CameraSample {
    pub p_film: Point2f,
    pub p_lens: Point2f,
    pub time: f32,
}
