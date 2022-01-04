package net.bluejekyll.wasmtime.proxy;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertArrayEquals;

import java.io.IOException;
import java.util.Optional;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import net.bluejekyll.wasmtime.TestUtil;
import net.bluejekyll.wasmtime.WasmEngine;
import net.bluejekyll.wasmtime.WasmFunction;
import net.bluejekyll.wasmtime.WasmInstance;
import net.bluejekyll.wasmtime.WasmLinker;
import net.bluejekyll.wasmtime.WasmStore;
import net.bluejekyll.wasmtime.WasmModule;
import net.bluejekyll.wasmtime.Wasmtime;
import net.bluejekyll.wasmtime.WasmtimeException;

public class WitTests {
    private Wasmtime wasmtime;
    private WasmEngine engine;
    private WasmModule module;
    private WasmStore store;
    private WasmLinker linker;

    @Before
    public void setup() throws WasmtimeException, IOException {
        this.wasmtime = new Wasmtime();
        this.engine = wasmtime.newWasmEngine();
        this.module = engine.newModule(TestUtil.WIT_LIB_PATH);
        System.out.println("slices compiled");
        assertNotNull(this.module);

        this.store = engine.newStore();
        this.linker = engine.newLinker();
    }

    @After
    public void tearDown() {
        this.linker.close();
        this.store.close();
        this.module.close();
        this.engine.close();
    }

    @Test
    public void test_play_with_bytes() throws Exception {
        WasmInstance instance = linker.instantiate(store, module);
        Optional<WasmFunction> func = instance.getFunction(store, "play_with_bytes");

        assertTrue("play_with_bytes isn't present in the module", func.isPresent());
        WasmFunction function = func.get();

        byte[] bytes = new byte[] { 5, 1, 2, 3 };

        byte[] ret = function.call(instance, store, byte[].class, bytes);
        assertNotNull(ret);
        assertEquals(bytes.length, ret.length);

        assertArrayEquals(ret, new byte[] { 6, 2, 3, 4 });
    }
}
