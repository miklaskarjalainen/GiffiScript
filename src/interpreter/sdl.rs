use std::ffi::{CString, c_char};
use sdl2_sys::{SDL_CreateRenderer, SDL_CreateWindow, SDL_RendererFlags, SDL_WindowFlags, SDL_WINDOWPOS_CENTERED_MASK, SDL_QuitEvent};

use crate::parser::{Parser, ParserToken};
use crate::interpreter::{Interpreter};
use crate::value::{Value};


pub fn import_libs(interpreter: &mut Interpreter) {
    interpreter.declare_function(&"sdl_init".to_string(), &vec![
        ParserToken::CallNative(sdl_init)
    ]);
    interpreter.declare_function(&"sdl_quit".to_string(), &vec![
        ParserToken::CallNative(sdl_quit)
    ]);

    // Window stuff
    interpreter.declare_function(&"create_window".to_string(), &vec![
        ParserToken::CallNative(create_window)
    ]);
    interpreter.declare_function(&"destroy_window".to_string(), &vec![
        ParserToken::CallNative(destroy_window)
    ]);

    // Utils
    interpreter.declare_function(&"do_events".to_string(), &vec![
        ParserToken::CallNative(do_events)
    ]);
    interpreter.declare_function(&"get_key_scancode".to_string(), &vec![
        ParserToken::CallNative(get_key_scancode)
    ]);
    interpreter.declare_function(&"is_key_down".to_string(), &vec![
        ParserToken::CallNative(is_key_down)
    ]);

    // Renderer stuff
    interpreter.declare_function(&"destroy_renderer".to_string(), &vec![
        ParserToken::CallNative(destroy_renderer)
    ]);
    interpreter.declare_function(&"create_renderer".to_string(), &vec![
        ParserToken::CallNative(create_renderer)
    ]);
    interpreter.declare_function(&"render_present".to_string(), &vec![
        ParserToken::CallNative(render_present)
    ]);
    interpreter.declare_function(&"render_clear".to_string(), &vec![
        ParserToken::CallNative(render_clear)
    ]);
    interpreter.declare_function(&"set_render_draw_color".to_string(), &vec![
        ParserToken::CallNative(set_render_draw_color)
    ]);

    // Draw Funcs 
    interpreter.declare_function(&"render_draw_rect".to_string(), &vec![
        ParserToken::CallNative(render_draw_rect)
    ]);
    interpreter.declare_function(&"render_fill_rect".to_string(), &vec![
        ParserToken::CallNative(render_fill_rect)
    ]);
    interpreter.declare_function(&"render_draw_point".to_string(), &vec![
        ParserToken::CallNative(render_draw_point)
    ]);
}

fn sdl_init(_: *mut Interpreter) {
    unsafe {
        sdl2_sys::SDL_Init(sdl2_sys::SDL_INIT_EVERYTHING as u32);
    }
}

fn sdl_quit(_: *mut Interpreter) {
    unsafe {
        sdl2_sys::SDL_Quit();
    }
}

fn create_renderer(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let window_ptr = machine.pop().ptr() as *mut sdl2_sys::SDL_Window;
    
    // SDL2 Calls
    let renderer = unsafe { SDL_CreateRenderer(
        window_ptr, -1, SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32
    )};

    // Push
    machine.push(Value::Ptr(renderer as *mut u32));
}

fn destroy_renderer(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let renderer_ptr = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    // SDL2 Calls
    unsafe { 
        sdl2_sys::SDL_DestroyRenderer(renderer_ptr)
    };
}


fn create_window(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    // Get args
    let height = machine.pop().int() as i32;
    let width = machine.pop().int() as i32;
    let window_title = machine.pop().literal();
    
    let title_cstring = CString::new(window_title).unwrap();
    let title_ptr: *const c_char = title_cstring.as_ptr() as *const c_char;

    // SDL2 Calls
    let window = unsafe {
        SDL_CreateWindow( 
            title_ptr, 
            805240832, 805240832, 
            width, height, 
            sdl2_sys::SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32
        )
    };

    machine.push(Value::Ptr(window as *mut u32));
}

fn destroy_window(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let window_ptr = machine.pop().ptr() as *mut sdl2_sys::SDL_Window;

    // SDL2 Calls
    unsafe { 
        sdl2_sys::SDL_DestroyWindow(window_ptr)
    };
}

fn do_events(_: *mut Interpreter) {
    // SDL2 Calls
    let layout = std::alloc::Layout::new::<sdl2_sys::SDL_Event>();
    let events = unsafe { std::alloc::alloc(layout) as *mut sdl2_sys::SDL_Event } ;
    unsafe { 
        while sdl2_sys::SDL_PollEvent(events) != 0 {
            match events {
                _ => {}
            }
        }
    };
}

fn get_key_scancode(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };
    let key_name = machine.pop().literal();

    // SDL2 Calls
    let key_cstring = CString::new(key_name).unwrap();
    let key_ptr: *const c_char = key_cstring.as_ptr() as *const c_char;

    let scancode = unsafe {
        sdl2_sys::SDL_GetScancodeFromName(key_ptr)
    };
    machine.push(Value::Int(scancode as i64));
}

fn is_key_down(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let scancode = machine.pop().int() as usize;

    // SDL2 Calls
    use core::ptr::null_mut;
    let is_down = unsafe {
        let keys = sdl2_sys::SDL_GetKeyboardState(null_mut());
        let new_addr = keys.add(scancode);
        *new_addr != 0
    };

    machine.push(Value::Boolean(is_down));
}

fn set_render_draw_color(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let a = machine.pop().int() as u8;
    let b = machine.pop().int() as u8;
    let g = machine.pop().int() as u8;
    let r = machine.pop().int() as u8;
    let renderer = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    // SDL2 Calls
    unsafe {
        sdl2_sys::SDL_SetRenderDrawColor(renderer, r, g, b, a);
    }
}


fn render_clear(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let renderer = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    // SDL2 Calls
    unsafe {
        sdl2_sys::SDL_RenderClear(renderer);
    }
}

fn render_present(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let renderer = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    // SDL2 Calls
    unsafe {
        sdl2_sys::SDL_RenderPresent(renderer);
    }
}

fn render_draw_rect(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let h = machine.pop().int() as i32;
    let w = machine.pop().int() as i32;
    let y = machine.pop().int() as i32;
    let x = machine.pop().int() as i32;
    let renderer = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    let rect = sdl2_sys::SDL_Rect{
        x: x,
        y: y,
        w: w,
        h: h
    };

    // SDL2 Calls
    unsafe {
        sdl2_sys::SDL_RenderDrawRect(renderer, &rect);
    }
}

fn render_fill_rect(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let h = machine.pop().int() as i32;
    let w = machine.pop().int() as i32;
    let y = machine.pop().int() as i32;
    let x = machine.pop().int() as i32;
    let renderer = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    let rect = sdl2_sys::SDL_Rect{
        x: x,
        y: y,
        w: w,
        h: h
    };

    // SDL2 Calls
    unsafe {
        sdl2_sys::SDL_RenderFillRect(renderer, &rect);
    }
}

fn render_draw_point(interpreter: *mut Interpreter) {
    let machine = unsafe { interpreter.as_mut().unwrap() };

    let y = machine.pop().int() as i32;
    let x = machine.pop().int() as i32;
    let renderer = machine.pop().ptr() as *mut sdl2_sys::SDL_Renderer;

    // SDL2 Calls
    unsafe {
        sdl2_sys::SDL_RenderDrawPoint(renderer, x, y);
    }
}

