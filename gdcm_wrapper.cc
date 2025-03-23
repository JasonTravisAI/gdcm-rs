#include "gdcmImageReader.h"
#include "gdcmImage.h"
#include "wrapper.h"

using namespace std;

struct PixelDataInternal c_decode_frames(
    char **i_buffer_ptr,
    size_t *i_buffer_lens,
    size_t i_buffer_len,
    u_int32_t dims[3],
    u_int32_t pi_type,
    u_int32_t ts_type,
    u_int16_t samples_per_pixel,
    u_int16_t bits_allocated,
    u_int16_t bits_stored,
    u_int16_t high_bit,
    u_int16_t pixel_representation)
{
    // Create fragment
    // We need a SmartPointer because of a GDCM bug
    // https://sourceforge.net/p/gdcm/mailman/gdcm-developers/thread/CB8517FD.82C8%25mkazanov%40gmail.com/
    gdcm::SmartPointer<gdcm::SequenceOfFragments> fragments = new gdcm::SequenceOfFragments();
    for (size_t i_buffer_idx = 0; i_buffer_idx < i_buffer_len; ++i_buffer_idx)
    {
        gdcm::Fragment fragment = gdcm::Fragment();
        fragment.SetByteValue(i_buffer_ptr[i_buffer_idx], gdcm::VL(i_buffer_lens[i_buffer_idx]));
        fragments->AddFragment(fragment);
    }

    // Create encapsulating DataElement
    gdcm::DataElement data_element = gdcm::DataElement(gdcm::Tag(0x7fe0, 0x0010));
    data_element.SetValue(*fragments);

    // TODO: Move this to a method
    gdcm::Image image = gdcm::Image();
    image.SetNumberOfDimensions(3);
    image.SetDimensions(dims);
    image.SetDataElement(data_element);
    image.SetPhotometricInterpretation(gdcm::PhotometricInterpretation(gdcm::PhotometricInterpretation::PIType(pi_type)));
    image.SetTransferSyntax(gdcm::TransferSyntax(gdcm::TransferSyntax::TSType(ts_type)));
    image.SetPixelFormat(gdcm::PixelFormat(samples_per_pixel, bits_allocated, bits_stored, high_bit, pixel_representation));

    struct PixelDataInternal outputStruct;
    size_t length = image.GetBufferLength();
    outputStruct.buffer = (char *)malloc(length);
    if (!image.GetBuffer(outputStruct.buffer))
    {
        outputStruct.status = 1;
        return outputStruct;
    }
    outputStruct.size = length;
    outputStruct.status = 0;
    return outputStruct;
}

void c_free_buffer(char *buffer_ptr)
{
    free(buffer_ptr);
}
