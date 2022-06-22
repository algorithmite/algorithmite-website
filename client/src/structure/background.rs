use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};
use weblog::console_log;
use yew::{html::Scope, prelude::*, props, Properties};

pub enum Msg {
    Render(f64),
    MouseMove(i32, i32),
}

pub struct Background {
    gl: Option<WebGlRenderingContext>,
    node_ref: NodeRef,
    _render_loop: Option<AnimationFrame>,
    current_x: f32,
    current_y: f32,
}

#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    pub background_width: i32,
    pub background_height: i32,
}

impl Component for Background {
    type Message = Msg;
    type Properties = BackgroundProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            gl: None,
            node_ref: NodeRef::default(),
            _render_loop: None,
            current_x: 0.0,
            current_y: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Render(timestamp) => {
                // Render functions are likely to get quite large, so it is good practice to split
                // it into it's own function rather than keeping it inline in the update match
                // case. This also allows for updating other UI elements that may be rendered in
                // the DOM like a framerate counter, or other overlaid textual elements.
                self.render_gl(timestamp, ctx.link());
                false
            }
            Msg::MouseMove(x, y) => {
                self.current_x = x as f32 / ctx.props().background_width as f32;
                self.current_y = y as f32 / ctx.props().background_height as f32;
                console_log!(format!("{}, {}", self.current_x, self.current_y));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                onmousemove={
                    ctx
                    .link()
                    .callback(|event: MouseEvent| Msg::MouseMove(event.client_x(), event.client_y()))
                }
                id="background-canvas"
                ref={self.node_ref.clone()}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.

        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: WebGlRenderingContext = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.gl = Some(gl);

        // In a more complex use-case, there will be additional WebGL initialization that should be
        // done here, such as enabling or disabling depth testing, depth functions, face
        // culling etc.

        if first_render {
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |time| link.send_message(Msg::Render(time)))
            };

            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            self._render_loop = Some(handle);
        }
    }
}

impl Background {
    fn render_gl(&mut self, timestamp: f64, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = include_str!("./../shader/background.vert");
        let frag_code = include_str!("./../shader/background.frag");

        // This list of vertices will draw two triangles to cover the entire canvas.
        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];
        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &verts,
            WebGlRenderingContext::STATIC_DRAW,
        );

        let vert_shader = gl
            .create_shader(WebGlRenderingContext::VERTEX_SHADER)
            .unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl
            .create_shader(WebGlRenderingContext::FRAGMENT_SHADER)
            .unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(time.as_ref(), timestamp as f32);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |time| link.send_message(Msg::Render(time)))
        };

        // A reference to the new handle must be retained for the next render to run.
        self._render_loop = Some(handle);
    }
}

impl BackgroundProps {
    pub fn new_background_with_size(background_width: i32, background_height: i32) -> Self {
        props! {BackgroundProps {
            background_width,
            background_height,
        }}
    }
}
