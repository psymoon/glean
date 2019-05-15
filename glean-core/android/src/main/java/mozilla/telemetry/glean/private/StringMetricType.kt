/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import com.sun.jna.StringArray
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.rust.LibGleanFFI

import mozilla.telemetry.glean.Dispatchers
// import mozilla.components.service.glean.storages.StringsStorageEngine
// import mozilla.components.support.base.log.logger.Logger

/**
 * This implements the developer facing API for recording string metrics.
 *
 * Instances of this class type are automatically generated by the parsers at build time,
 * allowing developers to record values that were previously registered in the metrics.yaml file.
 *
 * The string API only exposes the [set] method, which takes care of validating the input
 * data and making sure that limits are enforced.
 */
class StringMetricType(
    disabled: Boolean,
    category: String,
    lifetime: Lifetime,
    name: String,
    val sendInPings: List<String>
) {
    // private val logger = Logger("glean/StringMetricType")

    private var handle: Long

    init {
        println("New String: $category.$name")

        val ffiPingsList = StringArray(sendInPings.toTypedArray(), "utf-8")
        this.handle = LibGleanFFI.INSTANCE.glean_new_string_metric(
                category = category,
                name = name,
                send_in_pings = ffiPingsList,
                send_in_pings_len = sendInPings.size,
                lifetime = lifetime.ordinal,
                disabled = if (disabled) { 1 } else { 0 })
    }

    /**
     * Set a string value.
     *
     * @param value This is a user defined string value. If the length of the string exceeds
     *              the maximum length, it will be truncated.
     */
    fun set(value: String) {
        /*if (!shouldRecord(logger)) {
            return
        }*/

        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            setSync(value)
        }
    }

    /**
     * Internal only, synchronous API for setting a string value.
     */
    internal fun setSync(value: String) {
        LibGleanFFI.INSTANCE.glean_string_set(Glean.handle, this.handle, value)
    }

    /**
     * Tests whether a value is stored for the metric for testing purposes only. This function will
     * attempt to await the last task (if any) writing to the the metric's storage engine before
     * returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return true if metric value exists, otherwise false
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testHasValue(pingName: String = sendInPings.first()): Boolean {
        /*@Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return StringsStorageEngine.getSnapshot(pingName, false)?.get(identifier) != null*/
        assert(false, { "Testing API not implementated for StringMetricType" })
        return false
    }

    /**
     * Returns the stored value for testing purposes only. This function will attempt to await the
     * last task (if any) writing to the the metric's storage engine before returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return value of the stored metric
     * @throws [NullPointerException] if no value is stored
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetValue(pingName: String = sendInPings.first()): String {
        /*@Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return StringsStorageEngine.getSnapshot(pingName, false)!![identifier]!!*/
        assert(false, { "Testing API not implementated for StringMetricType" })
        return "asd"
    }
}
