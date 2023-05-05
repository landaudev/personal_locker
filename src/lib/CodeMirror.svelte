<script context="module">
import { EditorView, minimalSetup, basicSetup } from 'codemirror'
import { StateEffect } from '@codemirror/state'
export { minimalSetup, basicSetup }
</script>

<script>
import { onMount, onDestroy, createEventDispatcher } from 'svelte'
const dispatch = createEventDispatcher()

let dom

let _mounted = false
onMount(() => {
  _mounted = true
  return () => { _mounted = false }
})

export let view = null
export let doc
export let verbose = false
let _docCached = null
function _setText(text) {
  view.dispatch({
    changes: {from: 0, to: view.state.doc.length, insert: text},
  })
}

const subscribers = new Set()
export const docStore = {
  ready: () => (view !== null),
  subscribe(cb) {
    subscribers.add(cb)

    if (!this.ready()) {
      cb(null)
    } else {
      if (_docCached == null) {
        _docCached = view.state.doc.toString()
      }
      cb(_docCached)
    }

    return () => void subscribers.delete(cb)
  },
  set(newValue) {
    if (!_mounted) {
      throw new Error('Cannot set docStore when the component is not mounted.')
    }

    const inited = _initEditorView(newValue)
    if (!inited) _setText(newValue)
  },
}

export let extensions = minimalSetup

function _reconfigureExtensions() {
  if (view === null) return
  view.dispatch({
    effects: StateEffect.reconfigure.of(extensions),
  })
}

$: extensions, _reconfigureExtensions()

function _editorTxHandler(tr) {
  this.update([tr])

  if (verbose) {
    dispatch('update', tr)
  }

  if (tr.docChanged) {
    _docCached = null
    if (subscribers.size) {
      dispatchDocStore(_docCached = tr.newDoc.toString())
    }
    dispatch('change', {view: this, tr})
  }
}

function dispatchDocStore(s) {
  for (const cb of subscribers) {
    cb(s)
  }
}

function _initEditorView(initialDoc) {
  if (view !== null) {
    return false
  }

  view = new EditorView({
    doc: initialDoc,
    extensions,
    parent: dom,
    dispatch: _editorTxHandler,
  })
  return true
}

$: if (_mounted && doc !== undefined) {
  dispatchDocStore(doc)
}

onDestroy(() => {
  if (view !== null) {
    view.destroy()
  }
})
</script>

<div class="codemirror" bind:this={dom}>
</div>
