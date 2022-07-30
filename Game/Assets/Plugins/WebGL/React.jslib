mergeInto(LibraryManager.library, {
    onLeaveMatch: function () {
        window.dispatchReactUnityEvent(
            "onLeaveMatch"
        )
    },
    updateSelectedMovement: function (movement) {
        window.dispatchReactUnityEvent(
            "updateSelectedMovement", UTF8ToString(movement)
        )
    }
})